import { createFileRoute } from '@tanstack/react-router'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Users</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-muted-foreground">User management will be implemented here.</p>
      </CardContent>
    </Card>
  )
}
