import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { DragDropIcon, Search01Icon } from '@hugeicons/core-free-icons'
import { PERMISSION_CATEGORIES } from '../utils/constants'
import type { PermissionEnum } from '@/lib/api/types.gen'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'

interface PermissionPaletteProps {
  onDrag?: (permission: PermissionEnum) => void
}

export function PermissionPalette({ onDrag }: PermissionPaletteProps) {
  const [search, setSearch] = React.useState('')

  const filteredCategories: typeof PERMISSION_CATEGORIES =
    PERMISSION_CATEGORIES.map((category) => ({
      ...category,
      permissions: category.permissions.filter((p) =>
        p.toLowerCase().includes(search.toLowerCase()),
      ),
    })).filter((category) => category.permissions.length > 0)

  const handleDragStart = (e: React.DragEvent, permission: PermissionEnum) => {
    e.dataTransfer.setData('permission', permission)
    onDrag?.(permission)
  }

  return (
    <div className="flex h-full flex-col gap-4 border rounded-lg p-4 bg-muted/10">
      <div className="flex items-center gap-2">
        <div className="relative flex-1">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-2.5 top-2.5 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search permissions..."
            className="pl-9"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </div>

      <ScrollArea className="flex-1">
        <Accordion
          multiple
          defaultValue={PERMISSION_CATEGORIES.map((c) => c.name)}
        >
          {filteredCategories.map((category) => (
            <AccordionItem key={category.name} value={category.name}>
              <AccordionTrigger className="py-2 hover:no-underline">
                <span className="text-sm font-semibold">{category.name}</span>
              </AccordionTrigger>
              <AccordionContent>
                <div className="flex flex-wrap gap-2 pt-1">
                  {category.permissions.map((permission) => (
                    <div
                      key={permission}
                      draggable
                      onDragStart={(e) => handleDragStart(e, permission)}
                      className="cursor-grab active:cursor-grabbing"
                    >
                      <Badge
                        variant="secondary"
                        className="flex items-center gap-1.5 px-2 py-1 select-none"
                      >
                        <HugeiconsIcon
                          icon={DragDropIcon}
                          className="size-3 text-muted-foreground"
                        />
                        {permission}
                      </Badge>
                    </div>
                  ))}
                </div>
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      </ScrollArea>
    </div>
  )
}
