import { AppSidebar } from "@/components/app-sidebar"
import { ServiceViewport } from "@/components/service-viewport"
import { Button } from "@/components/ui/button"
import { SidebarInset, SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { useInstancesStore } from "@/store/instances"

function App() {
  const openInstance = useInstancesStore((state) => state.openInstance)
  const instanceCount = useInstancesStore((state) => state.instances.length)

  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1" />
          <Button
            size="sm"
            onClick={() => openInstance("isolation-test", `Test ${instanceCount + 1}`)}
          >
            Add test instance
          </Button>
        </header>
        <div className="relative flex-1">
          <ServiceViewport />
        </div>
      </SidebarInset>
    </SidebarProvider>
  )
}

export default App
