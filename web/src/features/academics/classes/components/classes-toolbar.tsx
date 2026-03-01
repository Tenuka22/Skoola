import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Download01Icon,
  FilterIcon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { useClassesStore } from '../store'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { HStack } from '@/components/primitives'
import {
  getAllAcademicYearsOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface ClassesToolbarProps {
  onExport: () => void
}

export function ClassesToolbar({ onExport }: ClassesToolbarProps) {
  const {
    search,
    setSearch,
    setIsCreateClassOpen,
    gradeId,
    setGradeId,
    academicYearId,
    setAcademicYearId,
  } = useClassesStore()

  const { data: academicYears } = useQuery({
    ...getAllAcademicYearsOptions({ client: authClient }),
  })

  const { data: gradeLevels } = useQuery({
    ...getAllGradeLevelsOptions({ client: authClient }),
  })

  return (
    <HStack justify="between">
      <HStack gap={4}>
        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search classes..."
            className="w-72 pl-9"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <HStack gap={2}>
          <Select
            value={academicYearId ?? 'all'}
            onValueChange={(value) =>
              setAcademicYearId(value === 'all' ? null : value)
            }
          >
            <SelectTrigger className="w-[180px]">
              <HStack gap={1} p={0}>
                <HugeiconsIcon icon={FilterIcon} className="size-4" />
                <SelectValue placeholder="Academic Year" />
              </HStack>
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Academic Years</SelectItem>
              {academicYears?.data.map((year) => (
                <SelectItem key={year.id} value={year.id}>
                  {year.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Select
            value={gradeId ?? 'all'}
            onValueChange={(value) =>
              setGradeId(value === 'all' ? null : value)
            }
          >
            <SelectTrigger className="w-[180px]">
              <HStack gap={1} p={0}>
                <HugeiconsIcon icon={FilterIcon} className="size-4" />
                <SelectValue placeholder="Grade Level" />
              </HStack>
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Grade Levels</SelectItem>
              {gradeLevels?.data.map((grade) => (
                <SelectItem key={grade.id} value={grade.id}>
                  {grade.grade_name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </HStack>
      </HStack>

      <HStack gap={2}>
        <Button variant="outline" onClick={onExport}>
          <HugeiconsIcon icon={Download01Icon} className="size-4 mr-2" />
          Export
        </Button>
        <Button onClick={() => setIsCreateClassOpen(true)}>
          <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
          Add Class
        </Button>
      </HStack>
    </HStack>
  )
}
