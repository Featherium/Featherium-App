# Featherium — Sub-projeto 1: Fundação + POC de Isolamento de Multiwebview

Data: 2026-07-23
Status: Aprovado

## Contexto e objetivo

O Featherium é um hub desktop multiserviço (concorrente do Ferdium/Franz/Rambox) construído com
Tauri v2 + Rust + React/TypeScript, com dois critérios inegociáveis acima de qualquer outra
consideração: **leveza extrema de recursos** (webview nativo do SO, não Chromium empacotado) e
**segurança** (isolamento rígido de sessão, superfície de ataque mínima).

Este é o primeiro de uma série de sub-projetos independentes. Ele estabelece a base técnica e
prova, antes de qualquer outra feature, que o conceito central do produto — múltiplas instâncias
do mesmo serviço com sessões completamente isoladas entre si — é viável em Tauri v2.

Roadmap completo de sub-projetos (specs futuras, fora de escopo aqui):

1. **Fundação + POC de isolamento** (este documento)
2. Sistema de recipes (WhatsApp, Telegram, Discord, Gmail, Slack, URL customizada)
3. Notificações e badges (detecção de mensagens novas, notificações nativas do SO)
4. i18n (pt-BR, en, es, detecção automática do idioma do sistema)
5. Gerenciamento de memória (suspensão/descarregamento de webviews em segundo plano)
6. Segurança e updater (hardening final, armazenamento seguro de credenciais, updater assinado)
7. Empacotamento (instaladores Windows/Linux/macOS)

## Descoberta técnica que molda o design

Pesquisa na documentação e issues oficiais do Tauri v2 confirmou:

- Multiwebview (múltiplos `Webview`s filhos de uma `Window`) existe em Tauri v2, mas está atrás
  da feature flag `unstable` do crate `tauri`.
- Isolamento de sessão por webview via `WebviewBuilder::data_directory()` funciona nativamente em
  **Windows (WebView2)** e **Linux (WebKitGTK)**.
- **macOS não é suportado** pela API `data_directory()` no momento (limitação conhecida da
  wry/Tauri, rastreada em [tauri-apps/tauri#9285](https://github.com/tauri-apps/tauri/issues/9285),
  que propõe uma API futura de "Browser Profiles"). Isso é uma limitação de terceiros, não uma
  escolha de design nossa.

Consequência para este sub-projeto: o critério de aceite automatizado do isolamento cobre Windows
e Linux. No macOS, a limitação é documentada e validada apenas manualmente; o suporte completo
será revisitado no sub-projeto 6 (Segurança) quando a API de profiles do Tauri amadurecer.

## Abordagens consideradas

- **A — Uma janela principal com múltiplos child-webviews isolados (escolhida).** A `Window`
  principal hospeda o webview do shell (React/sidebar). Cada instância de serviço é um `Webview`
  filho adicional na mesma janela, cada um com seu próprio `data_directory`. Só o webview ativo
  fica visível/dimensionado sobre a área de conteúdo; os demais permanecem carregados em segundo
  plano. É o modelo usado por Ferdium/Franz, mas com webview nativo em vez de Chromium empacotado.
- **B — Janelas do SO separadas por instância.** Rejeitada: expõe múltiplas janelas do sistema
  operacional em vez de uma interface única, e coordenar posição/z-order entre janelas em 3
  window managers diferentes (Windows, GNOME/KDE, macOS) é frágil e foge do requisito de
  "interface única".
- **C — Um único webview que troca de `src` a cada troca de serviço.** Rejeitada: destruiria e
  recriaria o webview a cada troca, eliminando segundo plano, notificações de instâncias
  inativas e uso simultâneo de múltiplas contas — viola o requisito central do produto.

## Arquitetura

### Backend Rust (`src-tauri/`)

- `Cargo.toml` habilita a feature `unstable` do crate `tauri` (necessária para multiwebview).
- A `Window` principal hospeda o webview do shell React. Comandos Tauri tipados expõem o ciclo
  de vida das instâncias de serviço:
  - `open_service_instance(recipe_id: String, label: String) -> InstanceId`
  - `focus_service_instance(id: InstanceId)`
  - `close_service_instance(id: InstanceId)`
  - `resize_service_instance(id: InstanceId, bounds: Rect)`
- Cada instância recebe um diretório de dados próprio em
  `<app-data-dir>/profiles/<uuid-gerado-pelo-backend>/`, passado via `WebviewBuilder::data_directory()`.
  O UUID é sempre gerado no backend (`uuid::Uuid::new_v4()`), nunca derivado de input do usuário —
  isso previne path traversal na composição do caminho.
- Webviews de conteúdo (as instâncias) recebem **zero capabilities** do Tauri: nenhum comando IPC,
  nenhum acesso a `core:default`. Somente o webview do shell (React) tem a capability com os
  comandos acima. Isso implementa o requisito de que o conteúdo web carregado nunca acessa
  diretamente APIs do sistema.
- `tauri.conf.json`: CSP restritiva aplicada aos assets do shell (`self` + mínimo necessário).
  Não é possível impor CSP sobre a resposta HTTP de sites de terceiros (WhatsApp Web etc.) —
  a isolação desses depende do `data_directory` por webview, não de CSP. Essa distinção fica
  documentada para não criar uma falsa expectativa de proteção via CSP nas specs futuras.

### Frontend Shell (`src/`, React + TypeScript + Vite + Tailwind v4)

- Sidebar baseada no bloco shadcn/ui `sidebar-07` (`collapsible="icon"`), reaproveitando:
  - `TeamSwitcher` → `SpaceSwitcher` (agrupamentos futuros de instâncias, ex. "Pessoal"/"Trabalho")
  - `NavMain` → lista de instâncias abertas, com destaque da instância ativa
  - `NavUser` → rodapé placeholder (configurações/tema, implementado em sub-projeto futuro)
- `ServiceViewport`: componente que mede seu próprio bounding box no DOM e informa o backend (via
  `resize_service_instance`) para posicionar o child-webview nativo correspondente sobre essa
  área. Não é um `<iframe>` — é coordenação de retângulos entre o layout React e o webview nativo.
- Estado local (lista de instâncias abertas, instância ativa) em um store simples (Zustand ou
  Context API — decisão de implementação, sem impacto arquitetural).

## Critério de aceite / validação do isolamento

Uma página HTML de teste, embutida no repositório (não um serviço real), gera um
`crypto.randomUUID()` na primeira carga e grava em `localStorage` e em um cookie. O fluxo de
validação:

1. Abrir 2 instâncias dessa página de teste via a sidebar.
2. Confirmar visualmente que cada instância mostra um UUID diferente.
3. Um comando de debug temporário (existe só nesta fase, removido antes do sub-projeto de
   recipes) lê os dados persistidos de cada `data_directory` e confirma programaticamente que
   os dois UUIDs diferem.
4. Reiniciar o app e reabrir as mesmas instâncias — os UUIDs devem persistir e continuar
   diferentes entre si (prova que o isolamento sobrevive a restart, não é só em memória).

Passos 1–3 automatizados/validados em Windows e Linux. No macOS, validação manual apenas, com a
limitação de `data_directory` documentada (ver seção de descoberta técnica).

## Fora de escopo neste sub-projeto

Recipes reais de serviços, notificações/badges, i18n, suspensão/descarregamento agressivo de
memória em segundo plano, updater, empacotamento de instaladores. Todos endereçados nos
sub-projetos 2–7 listados acima.

## Testes

- Testes Rust (`cargo test`) para lógica pura: geração de path de profile por UUID, clamping de
  `Rect` nos limites da janela.
- Validação manual/scriptada do critério de aceite de isolamento (seção acima).
- Sem automação E2E de browser neste sub-projeto (fica para uma fase posterior, quando houver
  recipes reais para testar).

## Tratamento de erros

- Falha ao criar `data_directory` (permissão/disco cheio) → comando retorna erro tipado,
  exibido como toast no shell; a instância não é adicionada à lista.
- Colisão de `data_directory` entre instâncias é estruturalmente impossível, pois o path sempre
  usa um UUID gerado pelo backend.
