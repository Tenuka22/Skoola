import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Calendar02Icon,
  Download01Icon,
  Search01Icon,
  User02Icon,
} from '@hugeicons/core-free-icons'
import { useTimetablesStore } from '../store'
import type { AcademicYearResponse, ClassResponse, StaffResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { cn } from '@/lib/utils'

interface TimetablesToolbarProps {
  onExport: () => void
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  staff: Array<StaffResponse>
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
  selectedTeacherId: string | undefined
  setSelectedTeacherId: (id: string | undefined) => void
  selectedDayOfWeek: string | undefined
  setSelectedDayOfWeek: (day: string | undefined) => void
  viewMode: 'class' | 'teacher'
  setViewMode: (mode: 'class' | 'teacher') => void
}

export function TimetablesToolbar({
  onExport,
  academicYears,
  classes,
  staff,
  selectedAcademicYearId,
  setSelectedAcademicYearId,
  selectedClassId,
  setSelectedClassId,
  selectedTeacherId,
  setSelectedTeacherId,
  selectedDayOfWeek,
  setSelectedDayOfWeek,
  viewMode,
  setViewMode,
}: TimetablesToolbarProps) {
  const { search, setSearch, setIsCreateTimetableEntryOpen } = useTimetablesStore()

  const daysOfWeek = [
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
    'Sunday',
  ]

  return (
    <div className="flex flex-col gap-4 px-8 py-4">
      <div className="flex items-center justify-between">
        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search timetables..."
            className="w-72 pl-9"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        <div className="flex items-center gap-2">
          <ToggleGroup
            value={[viewMode]}
            onValueChange={(value: Array<any>) =>
              value[0] && setViewMode(value[0] as 'class' | 'teacher')
            }
            className="border p-1 rounded-lg bg-muted/50"
          >
            <ToggleGroupItem
              value="class"
              className={cn(
                "px-3 py-1.5 h-auto text-xs data-[state=on]:bg-primary data-[state=on]:text-primary-foreground",
                viewMode === 'class' && 'shadow-sm',
              )}
            >
              <HugeiconsIcon icon={Calendar02Icon} className="size-4 mr-2" />
              Class View
            </ToggleGroupItem>
            <ToggleGroupItem
              value="teacher"
              className={cn(
                "px-3 py-1.5 h-auto text-xs data-[state=on]:bg-primary data-[state=on]:text-primary-foreground",
                viewMode === 'teacher' && 'shadow-sm',
              )}
            >
              <HugeiconsIcon icon={User02Icon} className="size-4 mr-2" />
              Teacher View
            </ToggleGroupItem>
          </ToggleGroup>

          <Button variant="outline" onClick={onExport}>
            <HugeiconsIcon icon={Download01Icon} className="size-4 mr-2" />
            Export
          </Button>
          <Button onClick={() => setIsCreateTimetableEntryOpen(true)}>
            <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
            Add Entry
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
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
        </div>

        {viewMode === 'class' && (
          <div className="flex items-center gap-2">
            <Label htmlFor="class-filter" className="text-sm text-muted-foreground">
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
          </div>
        )}

        {viewMode === 'class' && (
          <div className="flex items-center gap-2">
            <Label htmlFor="day-filter" className="text-sm text-muted-foreground">
              Day:
            </Label>
            <Select
              value={selectedDayOfWeek}
              onValueChange={(val) => setSelectedDayOfWeek(val || undefined)}
            >
              <SelectTrigger id="day-filter" className="w-[180px]">
                <SelectValue placeholder="Select Day" />
              </SelectTrigger>
              <SelectContent>
                {daysOfWeek.map((day) => (
                  <SelectItem key={day} value={day}>
                    {day}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}

        {viewMode === 'teacher' && (
          <div className="flex items-center gap-2">
            <Label htmlFor="teacher-filter" className="text-sm text-muted-foreground">
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
          </div>
        )}
      </div>
    </div>
  )
}
