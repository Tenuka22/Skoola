

import { useMemo } from 'react';
import { HugeiconsIcon } from '@hugeicons/react';
import {
  Cancel01Icon,
  Logout01Icon,
  MoreHorizontalIcon,
  Tick01Icon,
} from '@hugeicons/core-free-icons';
import type { StaffAttendanceWithMember, StudentAttendanceWithMember } from '../types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { cn } from '@/lib/utils';


interface SummaryCardProps {
  title: string;
  icon: any;
  iconClassName?: string;
  items: Array<{
    label: string;
    value: number;
    change?: number;
    changeLabel?: string;
  }>;
}

const SummaryCard = ({ title, icon, iconClassName, items }: SummaryCardProps) => {
  return (
    <Card className="flex-1 border-none shadow-sm bg-background">
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <div className="flex items-center gap-2">
          <div className={cn("p-2 rounded-lg bg-muted/50", iconClassName)}>
            <HugeiconsIcon icon={icon} className="size-4" />
          </div>
          <CardTitle className="text-sm font-bold">{title}</CardTitle>
        </div>
        <HugeiconsIcon icon={MoreHorizontalIcon} className="size-4 text-muted-foreground cursor-pointer" />
      </CardHeader>
      <CardContent className="grid grid-cols-3 gap-4 pt-4">
        {items.map((item, index) => (
          <div key={index} className="space-y-1">
            <p className="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">
              {item.label}
            </p>
            <p className="text-xl font-black">{item.value}</p>
            {item.change !== undefined && (
              <p className={cn(
                "text-[10px] font-bold",
                item.change >= 0 ? "text-green-500" : "text-red-500"
              )}>
                {item.change >= 0 ? '+' : ''}{item.change} {item.changeLabel}
              </p>
            )}
          </div>
        ))}
      </CardContent>
    </Card>
  );
};

interface AttendanceSummaryCardsProps {
  attendanceRecords: Array<StaffAttendanceWithMember | StudentAttendanceWithMember>;
}

export const AttendanceSummaryCards = ({ attendanceRecords }: AttendanceSummaryCardsProps) => {
  const stats = useMemo(() => {
    let presentCount = 0;
    let lateCount = 0;
    let absentCount = 0;
    let excusedCount = 0;
    let halfDayCount = 0;

    attendanceRecords.forEach((record) => {
      switch (record.status) {
        case "Present":
          presentCount++;
          break;
        case "Late":
          lateCount++;
          break;
        case "Absent":
          absentCount++;
          break;
        case "Excused":
          excusedCount++;
          break;
        case "HalfDay":
          halfDayCount++;
          break;
        default:
          break;
      }
    });

    return {
      total: attendanceRecords.length,
      present: presentCount,
      late: lateCount,
      absent: absentCount,
      excused: excusedCount,
      halfDay: halfDayCount,
    };
  }, [attendanceRecords]);

  return (
    <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
      <SummaryCard
        title="Daily Summary"
        icon={Tick01Icon}
        iconClassName="text-green-500 bg-green-500/10"
        items={[
          { label: 'Total', value: stats.total },
          { label: 'Present', value: stats.present },
          { label: 'Late', value: stats.late },
        ]}
      />
      <SummaryCard
        title="Absent / Excused"
        icon={Cancel01Icon}
        iconClassName="text-red-500 bg-red-500/10"
        items={[
          { label: 'Absent', value: stats.absent },
          { label: 'Excused', value: stats.excused },
          { label: 'Half Day', value: stats.halfDay },
        ]}
      />
      <SummaryCard
        title="Overall Stats"
        icon={Logout01Icon}
        iconClassName="text-blue-500 bg-blue-500/10"
        items={[
          {
            label: 'Attendance %',
            value: stats.total > 0 ? Math.round((stats.present / stats.total) * 100) : 0,
          },
          // Placeholder for more detailed stats if needed later
        ]}
      />
    </div>
  );
};
