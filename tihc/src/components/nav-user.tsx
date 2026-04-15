import { useState } from "react"
import {
  IconDotsVertical,
  IconLogin2,
  IconLogout,
  IconRefresh,
} from "@tabler/icons-react"

import { clearGoogleAuth, refreshGoogleAuth, setGoogleAuth } from "@/lib/app/runtime"
import {
  isGoogleOAuthConfigured,
  refreshGoogleAuthSession,
  signInWithGoogle,
  signOutFromGoogle,
} from "@/lib/auth/google-oauth"
import type { CurrentUserRecord, GoogleAuthState } from "@/lib/chat/agent-types"
import { Button } from "@/components/ui/button"
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@/components/ui/avatar"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar"

type NavUserProps = {
  user: CurrentUserRecord
  googleAuth: GoogleAuthState | null
}

type AuthActionState = {
  status: "idle" | "running" | "error"
  message: string
}

const idleAuthActionState: AuthActionState = {
  status: "idle",
  message: "",
}

function initialsFor(label: string): string {
  return (
    label
      .split(/\s+/)
      .slice(0, 2)
      .map((chunk) => chunk[0]?.toUpperCase() ?? "")
      .join("") || "AN"
  )
}

export function NavUser({ user, googleAuth }: NavUserProps) {
  const { isMobile } = useSidebar()
  const [loginDialogOpen, setLoginDialogOpen] = useState(false)
  const [actionState, setActionState] = useState<AuthActionState>(idleAuthActionState)
  const fallbackLabel = initialsFor(user.displayName)
  const secondaryLabel = user.email || (user.authState === "anonymous" ? "未登录" : "No email")
  const oauthConfigured = isGoogleOAuthConfigured()
  const actionDisabled = actionState.status === "running"

  const handleGoogleSignIn = async () => {
    if (!oauthConfigured) {
      setActionState({
        status: "error",
        message:
          "Google OAuth is not configured. Set WXT_GOOGLE_OAUTH_CLIENT_ID or a browser-specific override.",
      })
      return
    }

    setActionState({
      status: "running",
      message: "Opening the Google sign-in flow...",
    })

    try {
      const nextGoogleAuth = await signInWithGoogle()
      setGoogleAuth(nextGoogleAuth)
      setActionState(idleAuthActionState)
      setLoginDialogOpen(false)
    } catch (error) {
      setActionState({
        status: "error",
        message: error instanceof Error && error.message ? error.message : "Google sign-in failed.",
      })
    }
  }

  const handleGoogleRefresh = async () => {
    setActionState({
      status: "running",
      message: "Refreshing the Google bearer token...",
    })

    try {
      const nextGoogleAuth = await refreshGoogleAuthSession()
      refreshGoogleAuth(nextGoogleAuth)
      setActionState(idleAuthActionState)
    } catch (error) {
      setActionState({
        status: "error",
        message: error instanceof Error && error.message ? error.message : "Google token refresh failed.",
      })
    }
  }

  const handleGoogleSignOut = async () => {
    setActionState({
      status: "running",
      message: "Signing out...",
    })

    try {
      if (googleAuth?.accessToken?.trim()) {
        await signOutFromGoogle(googleAuth.accessToken)
      }
    } finally {
      clearGoogleAuth()
      setActionState(idleAuthActionState)
    }
  }

  const sidebarButton = (
    <SidebarMenuButton
      size="lg"
      type="button"
      className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
    >
      <Avatar className="h-8 w-8 rounded-lg grayscale">
        <AvatarImage src="" alt={user.displayName} />
        <AvatarFallback className="rounded-lg">{fallbackLabel}</AvatarFallback>
      </Avatar>
      <div className="grid flex-1 text-left text-sm leading-tight">
        <span className="truncate font-medium">{user.displayName}</span>
        <span className="truncate text-xs text-muted-foreground">
          {secondaryLabel}
        </span>
      </div>
      <IconDotsVertical className="ml-auto size-4" />
    </SidebarMenuButton>
  )

  if (user.authState === "anonymous") {
    return (
      <SidebarMenu>
        <SidebarMenuItem>
          <Dialog
            open={loginDialogOpen}
            onOpenChange={(open) => {
              setLoginDialogOpen(open)
              if (!open) {
                setActionState(idleAuthActionState)
              }
            }}
          >
            <DialogTrigger asChild>
              {sidebarButton}
            </DialogTrigger>
            <DialogContent className="max-w-xl">
              <DialogHeader className="space-y-3 text-left">
                <DialogTitle className="text-2xl tracking-tight">
                  Sign in to unlock full features
                </DialogTitle>
                <DialogDescription className="text-sm leading-6">
                  Continue anonymously if you only need local browser storage. Sign in once to
                  sync TIHC to your Google identity.
                </DialogDescription>
              </DialogHeader>
              <div className="space-y-3 text-sm leading-6">
                <div className="rounded-xl border bg-muted/30 px-4 py-3">
                  <div>Cloud sync</div>
                  <div>Usage analytics</div>
                  <div>Personal LLM settings</div>
                </div>
                {actionState.message ? (
                  <div className="rounded-xl border bg-muted/30 px-4 py-3">
                    {actionState.message}
                  </div>
                ) : null}
              </div>
              <DialogFooter>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => setLoginDialogOpen(false)}
                >
                  Continue as Anonymous
                </Button>
                <Button
                  type="button"
                  disabled={actionDisabled}
                  onClick={() => void handleGoogleSignIn()}
                >
                  <IconLogin2 className="size-4" />
                  <span>Sign in with Google</span>
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </SidebarMenuItem>
      </SidebarMenu>
    )
  }

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            {sidebarButton}
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-(--radix-dropdown-menu-trigger-width) min-w-64 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuLabel className="space-y-2 p-3 font-normal">
              <div className="flex items-center gap-2 text-left text-sm">
                <Avatar className="h-8 w-8 rounded-lg">
                  <AvatarImage src="" alt={user.displayName} />
                  <AvatarFallback className="rounded-lg">{fallbackLabel}</AvatarFallback>
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">{user.displayName}</span>
                  <span className="truncate text-xs text-muted-foreground">
                    {secondaryLabel}
                  </span>
                </div>
              </div>
              {user.hostedDomain ? (
                <div className="text-xs text-muted-foreground">
                  Hosted domain: {user.hostedDomain}
                </div>
              ) : null}
              {actionState.message ? (
                <div className="rounded-xl border bg-muted/30 px-3 py-2 text-xs text-muted-foreground">
                  {actionState.message}
                </div>
              ) : null}
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              disabled={actionDisabled}
              onSelect={(event) => {
                event.preventDefault()
                void handleGoogleRefresh()
              }}
            >
              <IconRefresh />
              Refresh Google Token
            </DropdownMenuItem>
            <DropdownMenuItem
              disabled={actionDisabled}
              onSelect={(event) => {
                event.preventDefault()
                void handleGoogleSignOut()
              }}
            >
              <IconLogout />
              Sign out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
