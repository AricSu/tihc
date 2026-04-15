"use client"

import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"

export function CaseRenameDialog({
  caseTitle,
  open,
  onOpenChange,
  onRename,
}: {
  caseTitle: string
  open: boolean
  onOpenChange: (open: boolean) => void
  onRename: (nextTitle: string) => void
}) {
  const [draft, setDraft] = useState(caseTitle)

  useEffect(() => {
    if (!open) return
    setDraft(caseTitle)
  }, [caseTitle, open])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>Rename case</DialogTitle>
          <DialogDescription>Update the case name used across the sidebar and composer.</DialogDescription>
        </DialogHeader>
        <form
          className="space-y-4"
          onSubmit={(event) => {
            event.preventDefault()
            const nextTitle = draft.trim()
            if (!nextTitle) return
            onRename(nextTitle)
            onOpenChange(false)
          }}
        >
          <Input
            aria-label="Case name"
            autoFocus
            value={draft}
            onChange={(event) => setDraft(event.target.value)}
            placeholder="Case name"
          />
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={!draft.trim()}>
              Rename
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
