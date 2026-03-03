import { HugeiconsIcon } from '@hugeicons/react'
import {
  ArrowDown01Icon,
  ArrowUp01Icon,
  Sorting01Icon,
  ViewIcon,
} from '@hugeicons/core-free-icons'
import type { Column } from '@tanstack/react-table'

import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Badge } from '@/components/ui/badge'
import { HStack, Text } from '@/components/primitives'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

interface DataTableColumnHeaderProps<
  TData,
  TValue,
> extends React.HTMLAttributes<HTMLDivElement> {
  column: Column<TData, TValue>
  title: string
}

function ActionTooltip({
  children,
  label,
}: {
  children: React.ReactElement
  label: string
}) {
  return (
    <Tooltip>
      <TooltipTrigger render={children} />
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  )
}

export function DataTableColumnHeader<TData, TValue>({
  column,
  title,
  className,
}: DataTableColumnHeaderProps<TData, TValue>) {
  if (!column.getCanSort()) {
    return (
      <Text
        size="sm"
        className={cn('font-semibold text-foreground/80', className)}
      >
        {title}
      </Text>
    )
  }

  const isSorted = column.getIsSorted()
  const sortIndex = column.getSortIndex()

  return (
    <TooltipProvider>
      <HStack gap={1} className={cn(className)}>
        <DropdownMenu>
          <ActionTooltip label={`Sort or hide ${title}`}>
            <DropdownMenuTrigger
              render={
                <Button
                  variant="ghost"
                  size="sm"
                  className="-ml-3 h-8 data-[state=open]:bg-accent"
                >
                  <HStack gap={1} p={0}>
                    <span>{title}</span>
                    {isSorted === 'desc' ? (
                      <HugeiconsIcon
                        icon={ArrowDown01Icon}
                        className="h-4 w-4"
                      />
                    ) : isSorted === 'asc' ? (
                      <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
                    ) : (
                      <HugeiconsIcon icon={Sorting01Icon} className="h-4 w-4" />
                    )}
                    {sortIndex > -1 && (
                      <Badge
                        variant="secondary"
                        className="h-4 w-4 rounded-full p-0 text-[10px] flex items-center justify-center bg-primary text-primary-foreground"
                      >
                        {sortIndex + 1}
                      </Badge>
                    )}
                  </HStack>
                </Button>
              }
            />
          </ActionTooltip>
          <DropdownMenuContent align="start">
            <DropdownMenuItem onClick={() => column.toggleSorting(false, true)}>
              <HugeiconsIcon
                icon={ArrowUp01Icon}
                className="mr-2 h-3.5 w-3.5 text-muted-foreground/70"
              />
              Asc
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => column.toggleSorting(true, true)}>
              <HugeiconsIcon
                icon={ArrowDown01Icon}
                className="mr-2 h-3.5 w-3.5 text-muted-foreground/70"
              />
              Desc
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => column.clearSorting()}>
              <HugeiconsIcon
                icon={Sorting01Icon}
                className="mr-2 h-3.5 w-3.5 text-muted-foreground/70"
              />
              Clear
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => column.toggleVisibility(false)}>
              <HugeiconsIcon
                icon={ViewIcon}
                className="mr-2 h-3.5 w-3.5 text-muted-foreground/70"
              />
              Hide
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </HStack>
    </TooltipProvider>
  )
}
