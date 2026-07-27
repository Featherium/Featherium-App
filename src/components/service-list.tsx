"use client"

import { useState } from "react"
import { MoreVerticalIcon, XIcon } from "lucide-react"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import { InstanceSettingsSheet } from "@/components/instance-settings-sheet"
import { useInstancesStore } from "@/store/instances"

export function ServiceList() {
  const instances = useInstancesStore((state) => state.instances)
  const activeInstanceId = useInstancesStore((state) => state.activeInstanceId)
  const focusInstance = useInstancesStore((state) => state.focusInstance)
  const closeInstance = useInstancesStore((state) => state.closeInstance)
  const [settingsInstanceId, setSettingsInstanceId] = useState<string | null>(null)
  const settingsInstance = instances.find((instance) => instance.id === settingsInstanceId) ?? null

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
            <SidebarMenuAction
              showOnHover
              className="right-7"
              onClick={() => setSettingsInstanceId(instance.id)}
            >
              <MoreVerticalIcon />
            </SidebarMenuAction>
            <SidebarMenuAction showOnHover onClick={() => closeInstance(instance.id)}>
              <XIcon />
            </SidebarMenuAction>
          </SidebarMenuItem>
        ))}
      </SidebarMenu>
      <InstanceSettingsSheet
        instance={settingsInstance}
        onOpenChange={(open) => {
          if (!open) setSettingsInstanceId(null)
        }}
      />
    </SidebarGroup>
  )
}
