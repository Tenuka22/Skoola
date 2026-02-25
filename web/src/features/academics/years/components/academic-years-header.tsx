'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon } from '@hugeicons/core-free-icons'
import { useAcademicYearsStore } from '../store'
import { Button } from '@/components/ui/button'

export function AcademicYearsHeader() {
  const { setIsCreateYearOpen } = useAcademicYearsStore()
  return (
    <div className="flex items-center justify-between p-4">
      <div>
        <h1 className="text-2xl font-bold">Academic Years</h1>
        <p className="text-muted-foreground">
          Manage academic years and terms.
        </p>
      </div>
      <Button onClick={() => setIsCreateYearOpen(true)}>
        <HugeiconsIcon icon={Add01Icon} className="mr-2 size-4" />
        Add Academic Year
      </Button>
    </div>
  )
}
