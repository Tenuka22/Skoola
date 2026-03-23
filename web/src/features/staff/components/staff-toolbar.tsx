import * as React from 'react'
import { FileImportIcon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { StaffFloatingToolbar } from './staff-floating-toolbar'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

interface StaffToolbarProps {
  selectedStaff: Set<string>
  floating?: boolean
  onBulkDelete: () => void
  onBulkEdit: () => void
  onImportJSON?: (rows: unknown) => void
  onCreateStaff: () => void
}

export function StaffToolbar({
  selectedStaff,
  floating = false,
  onBulkDelete,
  onBulkEdit,
  onImportJSON,
  onCreateStaff,
}: StaffToolbarProps) {
  const fileInputRef = React.useRef<HTMLInputElement>(null)

  const handleImport = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    const reader = new FileReader()
    reader.onload = (e) => {
      try {
        const content = e.target?.result
        if (typeof content === 'string') {
          const rows: unknown = JSON.parse(content)
          if (Array.isArray(rows) && onImportJSON) {
            onImportJSON(rows)
          }
        }
      } catch {
        // CSV parsing would go here
      }
    }
    reader.readAsText(file)
    if (fileInputRef.current) {
      fileInputRef.current.value = ''
    }
  }

  if (floating) {
    return (
      <StaffFloatingToolbar
        selectedCount={selectedStaff.size}
        onBulkDelete={onBulkDelete}
        onBulkEdit={onBulkEdit}
      />
    )
  }

  return (
    <HStack gap={2} justify="end">
      <Button
        variant="outline"
        size="sm"
        onClick={() => fileInputRef.current?.click()}
      >
        <HugeiconsIcon icon={FileImportIcon} className="size-4 mr-2" />
        Import
      </Button>
      <input
        ref={fileInputRef}
        type="file"
        accept=".json,.csv"
        className="hidden"
        onChange={handleImport}
      />
      <Button size="sm" onClick={onCreateStaff}>
        <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
        Add Staff
      </Button>
    </HStack>
  )
}
