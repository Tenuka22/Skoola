'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete01Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { ButtonGroup } from '@/components/ui/button-group'

interface StaffBulkActionsToolbarProps {
  selectedStaff: Set<string>
  onBulkDelete: () => void
  onBulkEdit: () => void
}

export function StaffBulkActionsToolbar({
  selectedStaff,
  onBulkDelete,
  onBulkEdit,
}: StaffBulkActionsToolbarProps) {
  const isVisible = selectedStaff.size > 0

  return isVisible ? (
    <div className="fixed inset-x-0 bottom-4 z-50 mx-auto w-fit">
      <div className="bg-background/80 rounded-full p-2 backdrop-blur-md">
        <ButtonGroup>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <Button
                  variant="destructive"
                  className="rounded-full"
                  onClick={onBulkDelete}
                >
                  <HugeiconsIcon icon={Delete01Icon} className="size-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Bulk Delete</TooltipContent>
            </Tooltip>
          </TooltipProvider>

          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <Button
                  variant="outline"
                  className="rounded-full"
                  onClick={onBulkEdit}
                >
                  <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Bulk Edit</TooltipContent>
            </Tooltip>
          </TooltipProvider>

          <div className="flex h-full items-center justify-center rounded-full bg-primary/10 px-4 text-xs font-semibold text-primary">
            {selectedStaff.size} selected
          </div>
        </ButtonGroup>
      </div>
    </div>
  ) : null
}
