"use client"

import * as React from "react"
import { BriefcaseIcon, SettingsIcon, UserIcon } from "lucide-react"

import { GlobalSettingsDialog } from "@/components/global-settings-dialog"
import { NavUser } from "@/components/nav-user"
import { ServiceList } from "@/components/service-list"
import { SpaceSwitcher, type Space } from "@/components/space-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar"

const SAMPLE_SPACES: Space[] = [
  { name: "Personal", logo: <UserIcon />, description: "Personal accounts" },
  { name: "Work", logo: <BriefcaseIcon />, description: "Work accounts" },
]

const PLACEHOLDER_USER = {
  name: "Featherium",
  email: "you@example.com",
  avatar: "",
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const [settingsOpen, setSettingsOpen] = React.useState(false)

  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <SpaceSwitcher spaces={SAMPLE_SPACES} />
      </SidebarHeader>
      <SidebarContent>
        <ServiceList />
      </SidebarContent>
      <SidebarFooter>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton onClick={() => setSettingsOpen(true)} tooltip="Configurações">
              <SettingsIcon />
              <span>Configurações</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
        <NavUser user={PLACEHOLDER_USER} />
      </SidebarFooter>
      <SidebarRail />
      <GlobalSettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </Sidebar>
  )
}
