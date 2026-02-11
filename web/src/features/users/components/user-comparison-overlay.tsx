import { HugeiconsIcon } from '@hugeicons/react'
import {
  ChartBarLineIcon,
  Delete02Icon,
  PencilEdit01Icon,
  Shield01Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import type { User } from '../types'
import { Button } from '@/components/ui/button'

interface UserComparisonOverlayProps {
  selectedUsers: Set<string>
  onClear: () => void
  onBulkVerify: (verify: boolean) => void
  onBulkDelete: () => void
  onBulkEdit: () => void
  onBulkManagePermissions: () => void
  users?: Array<User>
}

export function UserComparisonOverlay({
  selectedUsers,
  onClear,
  onBulkVerify,
  onBulkDelete,
  onBulkEdit,
  onBulkManagePermissions,
  users,
}: UserComparisonOverlayProps) {
  if (selectedUsers.size <= 1) return null

  const selectedData = users?.filter((u) => selectedUsers.has(u.id)) || []
  const verifyRate = Math.round(
    (selectedData.filter((u) => u.is_verified).length / selectedUsers.size) *
      100,
  )

  const coreProvider =
    Object.entries(
      selectedData.reduce((acc: any, u) => {
        const d = u.email.split('@')[1]
        acc[d] = (acc[d] || 0) + 1
        return acc
      }, {}),
    ).sort((a: any, b: any) => b[1] - a[1])[0]?.[0] || 'Mixed'

  return (
    <div className="fixed bottom-10 left-1/2 z-50 flex -translate-x-1/2 items-center gap-8 rounded-3xl border border-primary/20 bg-background/90 p-7 shadow-[0_20px_50px_rgba(0,0,0,0.2)] backdrop-blur-2xl ring-1 ring-white/10 animate-in fade-in slide-in-from-bottom-12 duration-500">
      <div className="flex items-center gap-4 border-r border-border/50 pr-8">
        <div className="flex size-14 items-center justify-center rounded-2xl bg-primary text-primary-foreground shadow-xl shadow-primary/20">
          <HugeiconsIcon icon={ChartBarLineIcon} className="size-7" />
        </div>
        <div>
          <p className="text-[11px] font-black uppercase tracking-[0.2em] text-muted-foreground/60">
            Cohort Analysis
          </p>
          <h4 className="text-base font-black tracking-tight">
            {selectedUsers.size} Users Selected
          </h4>
        </div>
      </div>

      <div className="flex gap-10">
        <div className="space-y-1.5">
          <span className="text-[11px] font-black text-muted-foreground/60 uppercase tracking-widest">
            Verify Rate
          </span>
          <p className="text-2xl font-black text-primary tabular-nums">
            {verifyRate}%
          </p>
        </div>
        <div className="space-y-1.5">
          <span className="text-[11px] font-black text-muted-foreground/60 uppercase tracking-widest">
            Core Provider
          </span>
          <p className="text-base font-bold tracking-tight">{coreProvider}</p>
        </div>
      </div>

      <div className="flex items-center gap-2 border-l border-border/50 pl-8">
        <Button
          variant="outline"
          size="sm"
          className="h-10 rounded-xl gap-2 font-bold px-4 hover:bg-primary/10"
          onClick={onBulkEdit}
        >
          <HugeiconsIcon
            icon={PencilEdit01Icon}
            className="size-4 text-primary"
          />
          Bulk Edit
        </Button>

        <Button
          variant="outline"
          size="sm"
          className="h-10 rounded-xl gap-2 font-bold px-4 hover:bg-primary/10"
          onClick={onBulkManagePermissions}
        >
          <HugeiconsIcon icon={Shield01Icon} className="size-4 text-primary" />
          Permissions
        </Button>

        <Button
          variant="outline"
          size="sm"
          className="h-10 rounded-xl gap-2 font-bold px-4"
          onClick={() => onBulkVerify(true)}
        >
          <HugeiconsIcon icon={Tick01Icon} className="size-4 text-primary" />
          Verify All
        </Button>

        <Button
          variant="destructive"
          size="sm"
          className="h-10 rounded-xl gap-2 font-bold px-4"
          onClick={onBulkDelete}
        >
          <HugeiconsIcon icon={Delete02Icon} className="size-4" />
          Purge All
        </Button>

        <Button
          variant="ghost"
          size="sm"
          className="font-black uppercase text-[10px] tracking-widest h-10 px-4 rounded-xl"
          onClick={onClear}
        >
          Dismiss
        </Button>
      </div>
    </div>
  )
}
