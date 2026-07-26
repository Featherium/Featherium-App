"use client"

import * as React from "react"
import { BriefcaseIcon, UserIcon } from "lucide-react"

import { NavUser } from "@/components/nav-user"
import { ServiceList } from "@/components/service-list"
import { SpaceSwitcher, type Space } from "@/components/space-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
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
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <SpaceSwitcher spaces={SAMPLE_SPACES} />
      </SidebarHeader>
      <SidebarContent>
        <ServiceList />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={PLACEHOLDER_USER} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
