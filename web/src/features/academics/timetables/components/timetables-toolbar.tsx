import type { TimetableViewMode } from '../constants'
import type {
  AcademicYearResponse,
  ClassResponse,
  StaffResponse,
} from '@/lib/api/types.gen'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import { HStack } from '@/components/primitives'

interface TimetablesToolbarProps {
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  staff: Array<StaffResponse>
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
  selectedTeacherId: string | undefined
  setSelectedTeacherId: (id: string | undefined) => void
  viewMode: TimetableViewMode
}

export function TimetablesToolbar({
  academicYears,
  classes,
  staff,
  selectedAcademicYearId,
  setSelectedAcademicYearId,
  selectedClassId,
  setSelectedClassId,
  selectedTeacherId,
  setSelectedTeacherId,
  viewMode,
}: TimetablesToolbarProps) {
  return (
    <HStack gap={4} px={8} py={4}>
      <HStack gap={2}>
        <Label htmlFor="year-filter" className="text-sm text-muted-foreground">
          Academic Year:
        </Label>
        <Select
          value={selectedAcademicYearId}
          onValueChange={(val) => setSelectedAcademicYearId(val || undefined)}
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

      {viewMode === 'class' && (
        <HStack gap={2}>
          <Label
            htmlFor="class-filter"
            className="text-sm text-muted-foreground"
          >
            Class:
          </Label>
          <Select
            value={selectedClassId}
            onValueChange={(val) => setSelectedClassId(val || undefined)}
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
      )}

      {viewMode === 'teacher' && (
        <HStack gap={2}>
          <Label
            htmlFor="teacher-filter"
            className="text-sm text-muted-foreground"
          >
            Teacher:
          </Label>
          <Select
            value={selectedTeacherId}
            onValueChange={(val) => setSelectedTeacherId(val || undefined)}
          >
            <SelectTrigger id="teacher-filter" className="w-[180px]">
              <SelectValue placeholder="Select Teacher" />
            </SelectTrigger>
            <SelectContent>
              {staff.map((teacher) => (
                <SelectItem key={teacher.id} value={teacher.id}>
                  {teacher.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </HStack>
      )}
    </HStack>
  )
}
