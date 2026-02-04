
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

interface StaffProfileProps {
  staffId: string;
  // TODO: Define props for staff data
}

export const StaffProfile = ({ staffId }: StaffProfileProps) => {
  // Placeholder staff data
  const staff = {
    id: staffId,
    name: "John Doe",
    email: "john.doe@example.com",
    role: "Teacher",
    department: "Science",
    // ... other profile details
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Staff Profile: {staff.name}</CardTitle>
          <CardDescription>Details for {staff.name} ({staff.role})</CardDescription>
        </CardHeader>
        <CardContent>
          {/* Basic overview info */}
          <p>Email: {staff.email}</p>
          <p>Department: {staff.department}</p>
        </CardContent>
      </Card>

      <Tabs defaultValue="overview" className="w-full">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="classes">Classes</TabsTrigger>
          <TabsTrigger value="attendance">Attendance</TabsTrigger>
          <TabsTrigger value="payroll">Payroll</TabsTrigger>
        </TabsList>
        <TabsContent value="overview">
          <Card>
            <CardHeader><CardTitle>Overview Details</CardTitle></CardHeader>
            <CardContent>
              <p>Full overview of staff member.</p>
              {/* More detailed overview content */}
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="classes">
          <Card>
            <CardHeader><CardTitle>Classes Assigned</CardTitle></CardHeader>
            <CardContent>
              <p>List of classes and subjects taught.</p>
              {/* Classes content */}
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="attendance">
          <Card>
            <CardHeader><CardTitle>Attendance Records</CardTitle></CardHeader>
            <CardContent>
              <p>Staff attendance history.</p>
              {/* Attendance content */}
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="payroll">
          <Card>
            <CardHeader><CardTitle>Payroll Information</CardTitle></CardHeader>
            <CardContent>
              <p>Payroll details and history.</p>
              {/* Payroll content */}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
};
