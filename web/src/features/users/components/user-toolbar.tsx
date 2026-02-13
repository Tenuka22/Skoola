'use client'

import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  PencilEdit01Icon,
  Shield01Icon,
  Tick01Icon,
  Delete02Icon,
  MoreHorizontalIcon,
} from '@hugeicons/core-free-icons'

import type { User } from '../types'
import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface UserToolbarProps {
  selectedUsers: Set<string>
  onClear: () => void
  onBulkVerify: (verify: boolean) => void
  onBulkDelete: () => void
  onBulkEdit: () => void
  onBulkManagePermissions: () => void
  users?: Array<User>
}

export function UserToolbar({
  selectedUsers,
  onClear,
  onBulkVerify,
  onBulkDelete,
  onBulkEdit,
  onBulkManagePermissions,
}: UserToolbarProps) {
  if (selectedUsers.size === 0) return null

  return (
    <div className="fixed bottom-6 left-1/2 z-50 -translate-x-1/2">
      <ButtonGroup>
        <Button variant="outline" disabled>
          {selectedUsers.size} Selected
        </Button>

        <Button variant="outline" onClick={onBulkEdit}>
          <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
          Edit
        </Button>

        <Button variant="outline" onClick={onBulkManagePermissions}>
          <HugeiconsIcon icon={Shield01Icon} className="mr-2 size-4" />
          Permissions
        </Button>

        <Button variant="outline" onClick={() => onBulkVerify(true)}>
          <HugeiconsIcon icon={Tick01Icon} className="mr-2 size-4" />
          Verify
        </Button>

        <Button onClick={onBulkDelete} variant="destructive">
          <HugeiconsIcon icon={Delete02Icon} />
          Delete
        </Button>
      </ButtonGroup>
    </div>
  )
}
