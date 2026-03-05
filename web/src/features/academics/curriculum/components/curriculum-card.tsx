import { HugeiconsIcon } from '@hugeicons/react' // This is the wrapper component for free icons
import {
  Book02Icon,
  Calendar01Icon,
  Cancel01Icon, // Free close icon
  Delete01Icon, // Free trash icon
  Edit01Icon, // Free edit icon
  LayerIcon,
  MoreVerticalIcon, // Free more vertical icon
  Tick01Icon, // Free check icon
} from '@hugeicons/core-free-icons'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { HStack, Stack, Text } from '@/components/primitives'

// Icons from @hugeicons/core-free-icons (these are objects, used with HugeiconsIcon)
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'

interface CurriculumCardProps {
  standard: CurriculumStandardResponse & {
    subject_name?: string
    grade_level_name?: string
  }
  onEdit: (standard: CurriculumStandardResponse) => void
  onDelete: (id: string) => void
}

export function CurriculumCard({
  standard,
  onEdit,
  onDelete,
}: CurriculumCardProps) {
  return (
    <Card>
      <CardHeader>
        <HStack justify="between" align="start">
          <Stack gap={1}>
            <CardTitle className="text-lg">{standard.standard_code}</CardTitle>
            <CardDescription>{standard.version_name}</CardDescription>
          </Stack>
          <DropdownMenu>
            <DropdownMenuTrigger>
              <Button variant="ghost" size="icon" className="size-8">
                <HugeiconsIcon icon={MoreVerticalIcon} className="size-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => onEdit(standard)}>
                <HugeiconsIcon icon={Edit01Icon} className="mr-2 size-4" />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => onDelete(standard.id)}
                className="text-destructive"
              >
                <HugeiconsIcon icon={Delete01Icon} className="mr-2 size-4" />
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </HStack>
      </CardHeader>
      <CardContent>
        <Stack gap={3}>
          <HStack gap={2} align="center">
            <HugeiconsIcon
              icon={Book02Icon}
              className="size-4 text-muted-foreground"
            />
            <Text className="font-medium">{standard.subject_name}</Text>
          </HStack>
          <HStack gap={2} align="center">
            <HugeiconsIcon
              icon={LayerIcon}
              className="size-4 text-muted-foreground"
            />
            <Text className="font-medium">{standard.grade_level_name}</Text>
          </HStack>
          <Text className="text-sm text-muted-foreground line-clamp-2">
            {standard.description}
          </Text>
        </Stack>
      </CardContent>
      <CardFooter>
        <HStack justify="between" className="w-full">
          <Badge variant={standard.is_active ? 'default' : 'destructive'}>
            {standard.is_active ? (
              <HugeiconsIcon icon={Tick01Icon} className="mr-1 size-3.5" />
            ) : (
              <HugeiconsIcon icon={Cancel01Icon} className="mr-1 size-3.5" />
            )}
            {standard.is_active ? 'Active' : 'Inactive'}
          </Badge>
          <HStack
            gap={2}
            align="center"
            className="text-xs text-muted-foreground"
          >
            <HugeiconsIcon icon={Calendar01Icon} className="size-3.5" />
            <span>
              {standard.start_date
                ? new Date(standard.start_date).toLocaleDateString()
                : 'N/A'}
            </span>
            <span>-</span>
            <span>
              {standard.end_date
                ? new Date(standard.end_date).toLocaleDateString()
                : 'N/A'}
            </span>
          </HStack>
        </HStack>
      </CardFooter>
    </Card>
  )
}
