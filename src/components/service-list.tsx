"use client"

import { XIcon } from "lucide-react"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import { useInstancesStore } from "@/store/instances"

export function ServiceList() {
  const instances = useInstancesStore((state) => state.instances)
  const activeInstanceId = useInstancesStore((state) => state.activeInstanceId)
  const focusInstance = useInstancesStore((state) => state.focusInstance)
  const closeInstance = useInstancesStore((state) => state.closeInstance)

  return (
    <SidebarGroup>
      <SidebarGroupLabel>Services</SidebarGroupLabel>
      <SidebarMenu>
        {instances.map((instance) => (
          <SidebarMenuItem key={instance.id}>
            <SidebarMenuButton
              isActive={instance.id === activeInstanceId}
              onClick={() => focusInstance(instance.id)}
              tooltip={instance.label}
            >
              <span>{instance.label}</span>
            </SidebarMenuButton>
            <SidebarMenuAction showOnHover onClick={() => closeInstance(instance.id)}>
              <XIcon />
            </SidebarMenuAction>
          </SidebarMenuItem>
        ))}
      </SidebarMenu>
    </SidebarGroup>
  )
}
