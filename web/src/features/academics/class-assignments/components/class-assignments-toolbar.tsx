import { HugeiconsIcon } from '@hugeicons/react'
import {
  Download01Icon,
  Plug01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useClassAssignmentsStore } from '../store'
import type { AcademicYearResponse, ClassResponse } from '@/lib/api/types.gen'
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

interface ClassAssignmentsToolbarProps {
  onExport: () => void
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
}

export function ClassAssignmentsToolbar({
  onExport,
  academicYears,
  classes,
  selectedAcademicYearId,
  setSelectedAcademicYearId,
  selectedClassId,
  setSelectedClassId,
}: ClassAssignmentsToolbarProps) {
  const { search, setSearch, setIsAssignTeacherOpen } =
    useClassAssignmentsStore()

  return (
    <div className="flex flex-col gap-4 px-8 py-4">
      <div className="flex items-center justify-between">
        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search assignments..."
            className="w-72 pl-9"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={onExport}>
            <HugeiconsIcon icon={Download01Icon} className="size-4 mr-2" />
            Export
          </Button>
          <Button onClick={() => setIsAssignTeacherOpen(true)}>
            <HugeiconsIcon icon={Plug01Icon} className="size-4 mr-2" />
            Assign Teacher
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Label
            htmlFor="class-filter"
            className="text-sm text-muted-foreground"
          >
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
        </div>

        <div className="flex items-center gap-2">
          <Label
            htmlFor="year-filter"
            className="text-sm text-muted-foreground"
          >
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
        </div>
      </div>
    </div>
  )
}
