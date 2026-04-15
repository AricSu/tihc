"use client"

import { useState } from "react"
import { IconFolder } from "@tabler/icons-react"
import { CaseRenameDialog } from "@/components/case-rename-dialog"
import { deleteCase, renameCase } from "@/lib/app/runtime"
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
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
    isActive?: boolean
    name: string
    onSelect?: () => void
    url: string
    status: string
    updatedAt: string
  }[]
}) {
  const [renameTarget, setRenameTarget] = useState<{ id: string; title: string } | null>(null)

  return (
    <>
      <SidebarGroup className="group-data-[collapsible=icon]:hidden">
        <SidebarGroupLabel>Cases</SidebarGroupLabel>
        <SidebarMenu>
          {items.length ? items.map((item) => (
            <SidebarMenuItem key={item.id}>
              <ContextMenu>
                <ContextMenuTrigger asChild>
                  <SidebarMenuButton asChild isActive={item.isActive}>
                    <a
                      href={item.url}
                      title={`Updated ${formatUpdatedAt(item.updatedAt)}`}
                      onClick={(event) => {
                        if (!item.onSelect) return
                        if (
                          event.defaultPrevented ||
                          event.button !== 0 ||
                          event.metaKey ||
                          event.altKey ||
                          event.ctrlKey ||
                          event.shiftKey
                        ) {
                          return
                        }
                        event.preventDefault()
                        item.onSelect()
                      }}
                    >
                      <IconFolder />
                      <span>{item.name}</span>
                    </a>
                  </SidebarMenuButton>
                </ContextMenuTrigger>
                <ContextMenuContent>
                  <ContextMenuItem onSelect={() => setRenameTarget({ id: item.id, title: item.name })}>
                    Rename case
                  </ContextMenuItem>
                  <ContextMenuItem variant="destructive" onSelect={() => deleteCase(item.id)}>
                    Delete case
                  </ContextMenuItem>
                </ContextMenuContent>
              </ContextMenu>
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
      <CaseRenameDialog
        caseTitle={renameTarget?.title ?? ""}
        open={!!renameTarget}
        onOpenChange={(open) => {
          if (open) return
          setRenameTarget(null)
        }}
        onRename={(nextTitle) => {
          if (!renameTarget) return
          renameCase(renameTarget.id, nextTitle)
        }}
      />
    </>
  )
}
