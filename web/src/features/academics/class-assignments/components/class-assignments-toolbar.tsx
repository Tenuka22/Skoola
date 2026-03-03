import type { AcademicYearResponse, ClassResponse } from '@/lib/api/types.gen'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import { HStack } from '@/components/primitives'

interface ClassAssignmentsToolbarProps {
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
}

export function ClassAssignmentsToolbar({
  academicYears,
  classes,
  selectedAcademicYearId,
  setSelectedAcademicYearId,
  selectedClassId,
  setSelectedClassId,
}: ClassAssignmentsToolbarProps) {
  return (
    <HStack gap={4} px={8} py={4}>
      <HStack gap={2}>
        <Label htmlFor="class-filter" className="text-sm text-muted-foreground">
          Class:
        </Label>
        <Select
          value={selectedClassId}
          onValueChange={(value) => {
            setSelectedClassId(value || undefined)
          }}
        >
          <SelectTrigger id="class-filter" className="w-[180px]">
            <SelectValue placeholder="Select Class" />
          </SelectTrigger>
          <SelectContent>
            {classes.map((cls) => (
              <SelectItem key={cls.id} value={cls.id}>
                {cls.section_name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </HStack>

      <HStack gap={2}>
        <Label htmlFor="year-filter" className="text-sm text-muted-foreground">
          Academic Year:
        </Label>
        <Select
          value={selectedAcademicYearId}
          onValueChange={(value) => {
            setSelectedAcademicYearId(value || undefined)
          }}
        >
          <SelectTrigger id="year-filter" className="w-[180px]">
            <SelectValue placeholder="Select Academic Year" />
          </SelectTrigger>
          <SelectContent>
            {academicYears.map((year) => (
              <SelectItem key={year.id} value={year.id}>
                {year.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </HStack>
    </HStack>
  )
}
