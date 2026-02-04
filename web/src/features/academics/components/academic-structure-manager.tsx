
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import * as Icons from '@hugeicons/react';

interface AcademicStructureManagerProps {
  // TODO: Define props for data and actions
}

export const AcademicStructureManager = ({ /* props */ }: AcademicStructureManagerProps) => {
  // Placeholder data for demonstration
  const academicYears = [
    { id: "ay1", name: "2024/2025", terms: [
      { id: "t1", name: "Term 1" },
      { id: "t2", name: "Term 2" },
    ]},
    { id: "ay2", name: "2025/2026", terms: [
      { id: "t3", name: "Term 1" },
    ]},
  ];

  const grades = [
    { id: "g1", name: "Grade 1" },
    { id: "g2", name: "Grade 2" },
  ];

  const classes = [
    { id: "c1", name: "1A", gradeId: "g1" },
    { id: "c2", name: "1B", gradeId: "g1" },
  ];

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Academic Structure Management</h2>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Academic Years & Terms</CardTitle>
          <Button size="sm"><Icons.Plus className="h-4 w-4 mr-2" />Add Year</Button>
        </CardHeader>
        <CardContent>
          {academicYears.map(year => (
            <div key={year.id} className="mb-4 pl-4 border-l-2">
              <h3 className="font-medium">Year: {year.name}</h3>
              <ul className="list-disc list-inside ml-4">
                {year.terms.map(term => (
                  <li key={term.id}>Term: {term.name}</li>
                ))}
              </ul>
            </div>
          ))}
          <Button variant="outline" size="sm"><Icons.Plus className="h-4 w-4 mr-2" />Add Term</Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Grades</CardTitle>
          <Button size="sm"><Icons.Plus className="h-4 w-4 mr-2" />Add Grade</Button>
        </CardHeader>
        <CardContent>
          <ul className="list-disc list-inside">
            {grades.map(grade => (
              <li key={grade.id}>{grade.name}</li>
            ))}
          </ul>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Classes</CardTitle>
          <Button size="sm"><Icons.Plus className="h-4 w-4 mr-2" />Add Class</Button>
        </CardHeader>
        <CardContent>
          <ul className="list-disc list-inside">
            {classes.map(cls => (
              <li key={cls.id}>{cls.name} (Grade {cls.gradeId})</li>
            ))}
          </ul>
        </CardContent>
      </Card>
    </div>
  );
};
