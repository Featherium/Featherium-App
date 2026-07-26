import { useEffect, useRef } from "react"

import { resizeServiceInstance } from "@/lib/tauri-commands"
import { useInstancesStore } from "@/store/instances"

export function ServiceViewport() {
  const containerRef = useRef<HTMLDivElement>(null)
  const activeInstanceId = useInstancesStore((state) => state.activeInstanceId)

  useEffect(() => {
    const container = containerRef.current
    if (!container || !activeInstanceId) return

    const reportBounds = () => {
      const rect = container.getBoundingClientRect()
      void resizeServiceInstance(activeInstanceId, {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
      })
    }

    reportBounds()
    const observer = new ResizeObserver(reportBounds)
    observer.observe(container)
    window.addEventListener("resize", reportBounds)

    return () => {
      observer.disconnect()
      window.removeEventListener("resize", reportBounds)
    }
  }, [activeInstanceId])

  return <div ref={containerRef} className="h-full w-full" />
}
