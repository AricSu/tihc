"use client"

import { IconFolder } from "@tabler/icons-react"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value)
  if (Number.isNaN(parsed)) return "Unknown update"

  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
  }).format(new Date(parsed))
}

export function NavDocuments({
  items,
}: {
  items: {
    id: string
    name: string
    url: string
    status: string
    updatedAt: string
  }[]
}) {
  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupLabel>Cases</SidebarGroupLabel>
      <SidebarMenu>
        {items.length ? items.map((item) => (
          <SidebarMenuItem key={item.id}>
            <SidebarMenuButton asChild>
              <a href={item.url} title={`Updated ${formatUpdatedAt(item.updatedAt)}`}>
                <IconFolder />
                <span>{item.name}</span>
              </a>
            </SidebarMenuButton>
            <SidebarMenuBadge>{item.status}</SidebarMenuBadge>
          </SidebarMenuItem>
        )) : (
          <SidebarMenuItem>
            <SidebarMenuButton className="text-sidebar-foreground/70" aria-disabled="true">
              <IconFolder className="text-sidebar-foreground/70" />
              <span>No cases yet</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        )}
      </SidebarMenu>
    </SidebarGroup>
  )
}
