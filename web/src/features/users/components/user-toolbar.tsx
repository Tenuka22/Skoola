'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  PencilEdit01Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'

import type { User } from '../types'
import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'
import { HStack } from '@/components/primitives'

interface UserToolbarProps {
  selectedUsers: Set<string>
  onBulkVerify: (verify: boolean) => void
  onBulkDelete: () => void
  onBulkEdit: () => void
  users?: Array<User>
}

export function UserToolbar({
  selectedUsers,
  onBulkVerify,
  onBulkDelete,
  onBulkEdit,
}: UserToolbarProps) {
  if (selectedUsers.size === 0) return null

  return (
    <div className="fixed bottom-6 left-1/2 z-50 -translate-x-1/2">
      <ButtonGroup>
        <Button variant="outline" disabled>
          {selectedUsers.size} Selected
        </Button>

        <Button variant="outline" onClick={onBulkEdit}>
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
            <span>Edit</span>
          </HStack>
        </Button>

        <Button variant="outline" onClick={() => onBulkVerify(true)}>
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={Tick01Icon} className="size-4" />
            <span>Verify</span>
          </HStack>
        </Button>

        <Button onClick={onBulkDelete} variant="destructive">
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={Delete02Icon} className="size-4" />
            <span>Delete</span>
          </HStack>
        </Button>
      </ButtonGroup>
    </div>
  )
}
