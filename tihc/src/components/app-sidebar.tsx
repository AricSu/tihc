import * as React from "react"
import {
  IconBook,
  IconChartBar,
  IconDashboard,
  IconFileWord,
  IconHelp,
  IconInnerShadowTop,
  IconPlugConnected,
  IconSettings,
} from "@tabler/icons-react"

import { NavDocuments } from "@/components/nav-documents"
import { NavMain } from "@/components/nav-main"
import { NavUser } from "@/components/nav-user"
import { Slider } from "@/components/ui/slider"
import { sortCasesByUpdatedAtDesc } from "@/lib/app/case-list"
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import type {
  AssistantReplyFontSize,
  CurrentUserRecord,
  GoogleAuthState,
} from "@/lib/chat/agent-types"

const data = {
  navSecondary: [
    {
      title: "Get Help",
      url: "#",
      icon: IconHelp,
    },
  ],
}

const replyFontSizeSliderSteps = ["small", "default", "large"] as const satisfies readonly AssistantReplyFontSize[]

function normalizeAssistantReplyFontSize(value: AssistantReplyFontSize | undefined): AssistantReplyFontSize {
  if (value === "small" || value === "large") {
    return value
  }

  return "default"
}

function replyFontSizeToSliderValue(value: AssistantReplyFontSize | undefined): number {
  return replyFontSizeSliderSteps.indexOf(normalizeAssistantReplyFontSize(value))
}

function sliderValueToReplyFontSize(value: number | undefined): AssistantReplyFontSize {
  const nextValue = typeof value === "number" ? replyFontSizeSliderSteps[value] : undefined
  return normalizeAssistantReplyFontSize(nextValue)
}

export function AppSidebar({
  activeCaseId = null,
  caseItems = [],
  currentUser = {
    id: null,
    authState: "anonymous",
    displayName: "匿名",
    email: "",
    hostedDomain: "",
  },
  googleAuth = null,
  onQuickCreate,
  onSelectCase,
  onNavigateSection,
  currentSection = "dashboard",
  assistantReplyFontSize = "default",
  onAssistantReplyFontSizeChange,
  ...props
}: React.ComponentProps<typeof Sidebar> & {
  activeCaseId?: string | null
  caseItems?: Array<{
    id: string
    title: string
    status: string
    updatedAt: string
  }>
  currentUser?: CurrentUserRecord
  googleAuth?: GoogleAuthState | null
  onQuickCreate?: () => void
  onSelectCase?: (caseId: string) => void
  onNavigateSection?: (section: "dashboard" | "usage" | "plugin" | "skills" | "llm") => void
  currentSection?: "dashboard" | "usage" | "plugin" | "skills" | "llm"
  assistantReplyFontSize?: AssistantReplyFontSize
  onAssistantReplyFontSizeChange?: (value: AssistantReplyFontSize) => void
}) {
  const navMain = [
    {
      title: "Dashboard",
      url: "?",
      icon: IconDashboard,
      isActive: currentSection === "dashboard",
      onSelect: () => onNavigateSection?.("dashboard"),
    },
    {
      title: "Docs",
      url: "https://www.askaric.com/en/tihc",
      icon: IconBook,
      target: "_blank",
      rel: "noopener noreferrer",
    },
    {
      title: "Usage",
      url: "?section=usage",
      icon: IconChartBar,
      isActive: currentSection === "usage",
      onSelect: () => onNavigateSection?.("usage"),
    },
    {
      title: "Plugins",
      url: "?section=plugin",
      icon: IconPlugConnected,
      isActive: currentSection === "plugin",
      onSelect: () => onNavigateSection?.("plugin"),
    },
    {
      title: "Skills",
      url: "?section=skills",
      icon: IconFileWord,
      isActive: currentSection === "skills",
      onSelect: () => onNavigateSection?.("skills"),
    },
    {
      title: "LLM",
      url: "?section=llm",
      icon: IconInnerShadowTop,
      isActive: currentSection === "llm",
      onSelect: () => onNavigateSection?.("llm"),
    },
  ]
  const documents = sortCasesByUpdatedAtDesc(caseItems).map((item) => ({
    id: item.id,
    isActive: item.id === activeCaseId,
    name: item.title,
    onSelect: () => onSelectCase?.(item.id),
    status: item.status,
    updatedAt: item.updatedAt,
    url: `#case-${item.id}`,
  }))
  const sliderValue = replyFontSizeToSliderValue(assistantReplyFontSize)
  const currentReplyFontSizeLabel =
    assistantReplyFontSize === "small"
      ? "Small"
      : assistantReplyFontSize === "large"
        ? "Large"
        : "Default"

  return (
    <Sidebar collapsible="offcanvas" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:p-1.5!"
            >
              <a href="#">
                <IconInnerShadowTop className="size-5!" />
                <span className="text-base font-semibold">tihc</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={navMain} onQuickCreate={onQuickCreate} />
        <NavDocuments items={documents} />
        <SidebarGroup className="mt-auto">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <Sheet>
                  <SheetTrigger asChild>
                    <SidebarMenuButton type="button">
                      <IconSettings />
                      <span>Settings</span>
                    </SidebarMenuButton>
                  </SheetTrigger>
                  <SheetContent side="right" className="gap-0">
                    <SheetHeader>
                      <SheetTitle>Sidebar Settings</SheetTitle>
                      <SheetDescription>
                        Adjust local display preferences without changing reply content or your composer.
                      </SheetDescription>
                    </SheetHeader>
                    <div className="flex flex-col gap-6 px-4 pb-6">
                      <div className="flex flex-col gap-3">
                        <div className="flex items-center justify-between gap-3">
                          <div className="flex flex-col gap-1">
                            <p className="text-sm font-medium">Reply font size</p>
                            <p className="text-sm text-muted-foreground">
                              Apply this only to assistant replies.
                            </p>
                          </div>
                          <span className="text-sm font-medium text-muted-foreground">
                            {currentReplyFontSizeLabel}
                          </span>
                        </div>
                        <Slider
                          aria-label="Reply font size"
                          min={0}
                          max={2}
                          step={1}
                          value={[sliderValue]}
                          onValueChange={(nextValue) => {
                            onAssistantReplyFontSizeChange?.(
                              sliderValueToReplyFontSize(nextValue[0]),
                            )
                          }}
                        />
                        <div className="flex items-center justify-between text-xs text-muted-foreground">
                          <span>Small</span>
                          <span>Default</span>
                          <span>Large</span>
                        </div>
                      </div>
                    </div>
                  </SheetContent>
                </Sheet>
              </SidebarMenuItem>
              {data.navSecondary.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <a href={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={currentUser} googleAuth={googleAuth} />
      </SidebarFooter>
    </Sidebar>
  )
}
