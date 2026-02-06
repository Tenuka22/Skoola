import React from 'react'
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Button } from '@/components/ui/button'

type ClassSubjectTeacherMatrixProps = object
// TODO: Define props for classes, subjects, teachers data

export const ClassSubjectTeacherMatrix = (
  _props: ClassSubjectTeacherMatrixProps,
) => {
  // Placeholder data
  const classes = [
    { id: 'c1', name: 'Grade 1A' },
    { id: 'c2', name: 'Grade 1B' },
  ]
  const subjects = [
    { id: 's1', name: 'Math' },
    { id: 's2', name: 'Science' },
  ]
  const teachers = [
    { id: 't1', name: 'Mr. Smith' },
    { id: 't2', name: 'Ms. Jane' },
  ]

  // Example assignments (Class ID, Subject ID, Teacher ID)
  const [assignments, setAssignments] = React.useState<
    Record<string, Record<string, string>>
  >({
    c1: { s1: 't1', s2: 't2' },
    c2: { s1: 't2', s2: 't1' },
  })

  const handleAssignmentChange = (
    classId: string,
    subjectId: string,
    teacherId: string,
  ) => {
    setAssignments((prev) => ({
      ...prev,
      [classId]: {
        ...(prev[classId] || {}),
        [subjectId]: teacherId,
      },
    }))
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">
        Class-Subject-Teacher Assignment Matrix
      </h2>
      <Table>
        <TableCaption>
          A list of class, subject, and teacher assignments.
        </TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[100px]">Class</TableHead>
            {subjects.map((subject) => (
              <TableHead key={subject.id}>{subject.name}</TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {classes.map((cls) => (
            <TableRow key={cls.id}>
              <TableCell className="font-medium">{cls.name}</TableCell>
              {subjects.map((subject) => (
                <TableCell key={subject.id}>
                  <Select
                    value={
                      (assignments[cls.id] &&
                        assignments[cls.id][subject.id]) ||
                      ''
                    }
                    onValueChange={(value) =>
                      handleAssignmentChange(cls.id, subject.id, value || '')
                    }
                  >
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Assign Teacher" />
                    </SelectTrigger>
                    <SelectContent>
                      {teachers.map((teacher) => (
                        <SelectItem key={teacher.id} value={teacher.id}>
                          {teacher.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Button>Save Assignments</Button>
    </div>
  )
}
