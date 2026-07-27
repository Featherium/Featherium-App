"use client"

import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import type { ServiceInstance } from "@/store/instances"
import { useInstancesStore } from "@/store/instances"

export function InstanceSettingsSheet({
  instance,
  onOpenChange,
}: {
  instance: ServiceInstance | null
  onOpenChange: (open: boolean) => void
}) {
  const setUserAgent = useInstancesStore((state) => state.setUserAgent)

  return (
    <Sheet open={instance !== null} onOpenChange={onOpenChange}>
      <SheetContent>
        <SheetHeader>
          <SheetTitle>{instance?.label ?? "Configurações"}</SheetTitle>
          <SheetDescription>Configurações desta instância.</SheetDescription>
        </SheetHeader>
        <div className="flex items-center justify-between gap-4 px-4">
          <div className="space-y-1">
            <Label htmlFor="native-user-agent">Usar user-agent nativo</Label>
            <p className="text-sm text-muted-foreground">
              O user-agent recomendado (Chrome desktop) evita telas de "navegador não suportado".
              O nativo pode deixar o WhatsApp Web com comportamento inconsistente. Trocar recarrega
              a página, mas a sessão continua logada.
            </p>
          </div>
          <Switch
            id="native-user-agent"
            checked={instance?.nativeUserAgent ?? false}
            onCheckedChange={(checked) => {
              if (instance) void setUserAgent(instance.id, checked)
            }}
          />
        </div>
      </SheetContent>
    </Sheet>
  )
}
