'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  Settings02Icon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import type { PermissionSet } from '../types'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'

interface RoleCardProps {
  permissionSet: PermissionSet
  userCount: number
  permissionCount: number
  onManage: (permissionSet: PermissionSet) => void
  onDelete?: () => void
}

export function RoleCard({
  permissionSet,
  userCount,
  permissionCount,
  onManage,
  onDelete,
}: RoleCardProps) {
  return (
    <Card className="group relative overflow-hidden border-none bg-background/50 shadow-xl ring-1 ring-border transition-all hover:shadow-2xl hover:ring-primary/20">
      <div className="absolute top-0 right-0 p-6 opacity-5 transition-opacity group-hover:opacity-10">
        <HugeiconsIcon icon={Settings02Icon} className="size-24 rotate-12" />
      </div>

      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <div className="flex items-center gap-3">
          <div className="flex size-10 items-center justify-center rounded-xl bg-primary/10 text-primary transition-colors group-hover:bg-primary group-hover:text-primary-foreground">
            <HugeiconsIcon icon={UserIcon} className="size-5" />
          </div>
          <div>
            <CardTitle className="text-lg font-black tracking-tight">
              {permissionSet.name}
            </CardTitle>
            <div className="flex items-center gap-2 mt-0.5">
              <Badge
                variant="secondary"
                className="h-4 px-1.5 text-[9px] font-black uppercase tracking-tighter"
              >
                {permissionCount} Permissions
              </Badge>
            </div>
          </div>
        </div>
        <div className="flex items-center gap-1">
          {onDelete && (
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8 rounded-lg text-destructive hover:bg-destructive/10 transition-colors"
              onClick={(e) => {
                e.stopPropagation()
                onDelete()
              }}
            >
              <HugeiconsIcon icon={Delete02Icon} className="size-4" />
            </Button>
          )}
          <Button
            variant="ghost"
            size="sm"
            className="h-8 rounded-lg text-[10px] font-black uppercase tracking-wider"
            onClick={() => onManage(permissionSet)}
          >
            Config
          </Button>
        </div>
      </CardHeader>

      <CardContent className="pt-4">
        <div className="flex items-center justify-between">
          <div className="flex -space-x-2">
            {[1, 2, 3].map((i) => (
              <div
                key={i}
                className="size-8 rounded-full border-2 border-background bg-muted ring-1 ring-border"
              />
            ))}
            {userCount > 3 && (
              <div className="flex size-8 items-center justify-center rounded-full border-2 border-background bg-muted text-[10px] font-black ring-1 ring-border">
                +{userCount - 3}
              </div>
            )}
          </div>
          <div className="flex items-center gap-2">
            <Badge
              variant="outline"
              className="h-6 rounded-lg px-2 text-[10px] font-bold text-green-500 bg-green-500/5 border-green-500/20"
            >
              Enabled
            </Badge>
          </div>
        </div>

        <Button
          className="mt-6 w-full h-11 rounded-xl bg-muted/50 border-none font-black uppercase tracking-widest text-[11px] transition-all hover:bg-primary hover:text-primary-foreground active:scale-[0.98]"
          onClick={() => onManage(permissionSet)}
        >
          <HugeiconsIcon icon={Settings02Icon} className="mr-2 size-4" />
          Manage Role
        </Button>
      </CardContent>
    </Card>
  )
}
