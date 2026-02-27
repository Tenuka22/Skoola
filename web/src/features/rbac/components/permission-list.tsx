import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { Search01Icon } from '@hugeicons/core-free-icons'
import { PERMISSION_CATEGORIES } from '../utils/constants'
import type { PermissionEnum } from '@/lib/api/types.gen'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'

interface PermissionListProps {
  assignedPermissions: PermissionEnum[]
  onToggle: (permission: PermissionEnum, checked: boolean) => void
  isReadOnly?: boolean
}

export function PermissionList({ 
  assignedPermissions, 
  onToggle,
  isReadOnly = false
}: PermissionListProps) {
  const [search, setSearch] = React.useState('')

  const filteredCategories = React.useMemo(() => {
    return PERMISSION_CATEGORIES.map((category) => ({
      ...category,
      permissions: category.permissions.filter((p) =>
        p.toLowerCase().includes(search.toLowerCase()),
      ),
    })).filter((category) => category.permissions.length > 0)
  }, [search])

  return (
    <div className="flex h-full flex-col gap-4">
      <div className="relative">
        <HugeiconsIcon
          icon={Search01Icon}
          className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
        />
        <Input
          placeholder="Search permissions..."
          className="pl-9 h-10"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      <ScrollArea className="flex-1 -mr-4 pr-4">
        <Accordion
          multiple
          defaultValue={PERMISSION_CATEGORIES.map((c) => c.name)}
          className="space-y-2"
        >
          {filteredCategories.map((category) => (
            <AccordionItem 
              key={category.name} 
              value={category.name}
              className="overflow-hidden"
            >
              <AccordionTrigger className="py-2 px-2 hover:no-underline transition-colors">
                <div className="flex items-center gap-2">
                  <span className="text-sm font-bold tracking-tight">{category.name}</span>
                  <span className="text-[10px] font-mono px-1.5 py-0.5 text-muted-foreground">
                    {category.permissions.length}
                  </span>
                </div>
              </AccordionTrigger>
              <AccordionContent className="px-2 pb-2 pt-0">
                <div className="grid grid-cols-1 gap-1.5 mt-2">
                  {category.permissions.map((permission) => {
                    const isChecked = assignedPermissions.includes(permission)
                    return (
                      <div
                        key={permission}
                        className="flex items-center gap-3 p-2 group"
                      >
                        <Checkbox
                          id={`perm-${permission}`}
                          checked={isChecked}
                          onCheckedChange={(checked) => 
                            onToggle(permission, !!checked)
                          }
                          disabled={isReadOnly}
                        />
                        <label
                          htmlFor={`perm-${permission}`}
                          className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 cursor-pointer flex-1 py-1"
                        >
                          {permission}
                        </label>
                      </div>
                    )
                  })}
                </div>
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      </ScrollArea>
    </div>
  )
}
