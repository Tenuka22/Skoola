import { HugeiconsIcon } from '@hugeicons/react'
import { Database01Icon } from '@hugeicons/core-free-icons'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

type SeedingToolProps = object
// TODO: Define props for triggering the seeding endpoint

export const SeedingTool = (_props: SeedingToolProps) => {
  const handleSeedDatabase = () => {
    alert('Simulating database seeding... (postDevSeed endpoint call)')
    // TODO: Implement actual API call to postDevSeed endpoint
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">
        Developer Tools - Database Seeding
      </h2>

      <Card>
        <CardHeader>
          <CardTitle>Database Seeding</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <p className="text-sm text-muted-foreground">
            This action will re-populate the development database with sample
            data. Use with caution.
          </p>
          <Button
            onClick={handleSeedDatabase}
            className="w-fit"
            variant="destructive"
          >
            <HugeiconsIcon icon={Database01Icon} className="h-4 w-4 mr-2" />
            Trigger Dev Seed
          </Button>
        </CardContent>
      </Card>
    </div>
  )
}
