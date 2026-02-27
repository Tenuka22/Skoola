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

interface PermissionListProps {
  assignedPermissions: Array<PermissionEnum>
  onToggle: (permission: PermissionEnum, checked: boolean) => void
  isReadOnly?: boolean
}

export function PermissionList({
  assignedPermissions,
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

  const defaultOpen = React.useMemo(
    () => PERMISSION_CATEGORIES.map((c) => c.name),
    [],
  )

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

      <ScrollArea className="flex-1 -mr-4 pr-4">
        <Accordion multiple defaultValue={defaultOpen} className="space-y-2">
          {filteredCategories.map((category) => (
            <AccordionItem
              key={category.name}
              value={category.name}
              className="border-none"
            >
              <AccordionTrigger className="py-2 px-2 hover:no-underline rounded-md hover:bg-muted/50 transition-colors">
                <HStack justify="between" className="w-full">
                  <HStack gap={2}>
                    <Text className="font-bold tracking-tight">
                      {category.name}
                    </Text>
                    <Badge variant="secondary" className="h-5">
                      {category.permissions.length}
                    </Badge>
                  </HStack>
                </HStack>
              </AccordionTrigger>
              <AccordionContent className="px-2 pb-2 pt-0">
                <Stack gap={1} className="mt-2">
                  {category.permissions.map((permission) => {
                    const isChecked = assignedPermissions.includes(permission)
                    return (
                      <HStack
                        key={permission}
                        align="center"
                        gap={3}
                        className="p-2 rounded-md hover:bg-muted/50"
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
                      </HStack>
                    )
                  })}
                </Stack>
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      </ScrollArea>
    </Stack>
  )
}
