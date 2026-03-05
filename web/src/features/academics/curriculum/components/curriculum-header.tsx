import { HugeiconsIcon } from '@hugeicons/react'
import { Book01Icon, PlusSignIcon } from '@hugeicons/core-free-icons'
import { HStack, Heading, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

interface CurriculumHeaderProps {
  total?: number
  onAdd: () => void
}

export function CurriculumHeader({ total, onAdd }: CurriculumHeaderProps) {
  return (
    <Stack gap={1}>
      <HStack className="justify-between items-start">
        <HStack align="center" gap={2}>
          <HugeiconsIcon icon={Book01Icon} className="size-6 text-primary" />
          <Heading size="h2">Curriculum Standards</Heading>
          {total !== undefined && (
            <Badge
              variant="secondary"
              className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
            >
              {total} Total
            </Badge>
          )}
        </HStack>
        <Button onClick={onAdd} size="sm" className="gap-2">
          <HugeiconsIcon icon={PlusSignIcon} className="size-4" />
          Add Standard
        </Button>
      </HStack>
      <Text muted as="p">
        Manage curriculum standards across different subjects, grades, and
        mediums.
      </Text>
    </Stack>
  )
}
