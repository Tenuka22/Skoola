import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Button } from '@/components/ui/button';

interface TimetableEditorProps {
  // TODO: Define props for timetable data (periods, days, classes, subjects, teachers)
}

export const TimetableEditor = ({ /* props */ }: TimetableEditorProps) => {
  // Placeholder data for a simple timetable grid
  const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
  const periods = ['Period 1 (9:00-10:00)', 'Period 2 (10:00-11:00)', 'Period 3 (11:00-12:00)'];

  // Example timetable data (Day -> Period -> Content)
  const [timetable, setTimetable] = React.useState<Record<string, Record<string, string>>>(
    {
      'Monday': {
        'Period 1 (9:00-10:00)': 'Math - Gr 1A - Mr. Smith',
        'Period 2 (10:00-11:00)': 'Science - Gr 1A - Ms. Jane',
      },
      'Tuesday': {
        'Period 1 (9:00-10:00)': 'History - Gr 1A - Mr. Smith',
      },
    }
  );

  const handleCellClick = (day: string, period: string) => {
    const newContent = prompt(`Enter content for ${day}, ${period}:`, timetable[day]?.[period] || '');
    if (newContent !== null) {
      setTimetable(prev => ({
        ...prev,
        [day]: {
          ...prev[day],
          [period]: newContent,
        },
      }));
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Timetable Editor</h2>

      <Card>
        <CardHeader>
          <CardTitle>School Timetable</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[150px]">Time/Day</TableHead>
                {days.map(day => (
                  <TableHead key={day} className="text-center">{day}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {periods.map(period => (
                <TableRow key={period}>
                  <TableCell className="font-medium">{period}</TableCell>
                  {days.map(day => (
                    <TableCell
                      key={day}
                      className="text-center border cursor-pointer hover:bg-muted"
                      onClick={() => handleCellClick(day, period)}
                    >
                      {timetable[day]?.[period] || ''}
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
          <Button className="mt-4">Save Timetable</Button>
        </CardContent>
      </Card>
    </div>
  );
};
