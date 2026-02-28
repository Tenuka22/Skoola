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
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'

interface PermissionListProps {
  assignedPermissions: Array<PermissionEnum>
  inheritedPermissions?: Array<{
    permission: PermissionEnum
    source: 'role' | 'set'
    sourceName?: string
  }>
  onToggle: (permission: PermissionEnum, checked: boolean) => void
  isReadOnly?: boolean
}

export function PermissionList({
  assignedPermissions,
  inheritedPermissions = [],
  onToggle,
  isReadOnly = false,
}: PermissionListProps) {
  const [search, setSearch] = React.useState('')

  const filteredCategories = React.useMemo(() => {
    if (!search) return PERMISSION_CATEGORIES

    return PERMISSION_CATEGORIES.map((category) => ({
      ...category,
      permissions: category.permissions.filter((p) =>
        p.toLowerCase().includes(search.toLowerCase()),
      ),
    })).filter((category) => category.permissions.length > 0)
  }, [search])

  const getInheritanceDetails = (permission: PermissionEnum) => {
    return inheritedPermissions.filter((ip) => ip.permission === permission)
  }

  return (
    <Stack gap={4} className="h-full">
      <Box className="relative">
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
      </Box>

      <Accordion>
        {filteredCategories.map((category) => (
          <AccordionItem
            key={category.name}
            value={category.name}
            className="border-none"
          >
            <AccordionTrigger className="hover:no-underline px-2 hover:bg-muted/50 items-center">
              <HStack justify="between" className="w-full" p={0}>
                <HStack gap={2}>
                  <Text muted>{category.name}</Text>
                  <Badge variant="secondary" className="h-5">
                    {category.permissions.length}
                  </Badge>
                </HStack>
              </HStack>
            </AccordionTrigger>
            <AccordionContent className="px-2 pb-2 pt-2">
              <Stack gap={1}>
                {category.permissions.map((permission) => {
                  const isDirectlyAssigned =
                    assignedPermissions.includes(permission)
                  const details = getInheritanceDetails(permission)
                  const isInherited = details.length > 0
                  const isChecked = isDirectlyAssigned || isInherited

                  return (
                    <HStack
                      key={permission}
                      align="center"
                      gap={3}
                      className={cn(
                        'p-2 rounded-md transition-colors',
                        isChecked ? 'bg-primary/5' : 'hover:bg-muted/50',
                      )}
                    >
                      <Checkbox
                        id={`perm-${permission}`}
                        checked={isChecked}
                        onCheckedChange={(checked) =>
                          onToggle(permission, !!checked)
                        }
                        indeterminate={isInherited}
                        disabled={isReadOnly || isInherited}
                      />
                      <Stack gap={0} className="flex-1 py-1">
                        <label
                          htmlFor={`perm-${permission}`}
                          className={cn(
                            'text-sm font-medium leading-none cursor-pointer',
                            isInherited && 'text-muted-foreground',
                            isReadOnly && 'cursor-default',
                          )}
                        >
                          {permission}
                        </label>
                        {isInherited && (
                          <HStack gap={1} className="flex-wrap pt-1">
                            {details.map((d, i) => (
                              <Badge
                                key={i}
                                variant="outline"
                                className="text-[10px] py-0 h-4 bg-muted/30 border-muted-foreground/20 text-muted-foreground font-normal"
                              >
                                {d.source === 'role'
                                  ? `Role: ${d.sourceName}`
                                  : `Set: ${d.sourceName}`}
                              </Badge>
                            ))}
                          </HStack>
                        )}
                      </Stack>
                      {isDirectlyAssigned && !isInherited && (
                        <Badge variant="outline">Direct</Badge>
                      )}
                    </HStack>
                  )
                })}
              </Stack>
            </AccordionContent>
          </AccordionItem>
        ))}
      </Accordion>
    </Stack>
  )
}
