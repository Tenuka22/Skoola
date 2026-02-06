import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

interface StudentProfileProps {
  studentId: string
  // TODO: Define props for student data
}

export const StudentProfile = ({ studentId }: StudentProfileProps) => {
  // Placeholder student data
  const student = {
    id: studentId,
    name: 'Alice Johnson',
    grade: '10',
    stream: 'A',
    // ... other profile details
  }

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Student Profile: {student.name}</CardTitle>
          <CardDescription>
            Details for {student.name} (Grade {student.grade})
          </CardDescription>
        </CardHeader>
        <CardContent>
          {/* Basic overview info */}
          <p>Grade: {student.grade}</p>
          <p>Stream: {student.stream}</p>
        </CardContent>
      </Card>

      <Tabs defaultValue="academic" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="academic">Academic</TabsTrigger>
          <TabsTrigger value="health">Health</TabsTrigger>
          <TabsTrigger value="guardians">Guardians</TabsTrigger>
        </TabsList>
        <TabsContent value="academic">
          <Card>
            <CardHeader>
              <CardTitle>Academic Information</CardTitle>
            </CardHeader>
            <CardContent>
              <p>Student's academic records, performance, etc.</p>
              {/* Academic content */}
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="health">
          <Card>
            <CardHeader>
              <CardTitle>Health Information</CardTitle>
            </CardHeader>
            <CardContent>
              <p>Student's medical history, allergies, emergency contacts.</p>
              {/* Health content */}
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="guardians">
          <Card>
            <CardHeader>
              <CardTitle>Guardians</CardTitle>
            </CardHeader>
            <CardContent>
              <p>Details of student's guardians/parents.</p>
              {/* Guardians content */}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
