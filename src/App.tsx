import { useEffect } from "react"

import { AppSidebar } from "@/components/app-sidebar"
import { ServiceViewport } from "@/components/service-viewport"
import { Button } from "@/components/ui/button"
import { SidebarInset, SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { TooltipProvider } from "@/components/ui/tooltip"
import { useInstancesStore } from "@/store/instances"

function App() {
  const hydrate = useInstancesStore((state) => state.hydrate)
  const openInstance = useInstancesStore((state) => state.openInstance)

  useEffect(() => {
    if (!useInstancesStore.getState().hydrated) {
      void hydrate()
    }
  }, [hydrate])

  return (
    <TooltipProvider>
      <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
          <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
            <SidebarTrigger className="-ml-1" />
            <Button size="sm" onClick={() => openInstance("whatsapp", "WhatsApp")}>
              Adicionar WhatsApp
            </Button>
          </header>
          <div className="relative flex-1">
            <ServiceViewport />
          </div>
        </SidebarInset>
      </SidebarProvider>
    </TooltipProvider>
  )
}

export default App
