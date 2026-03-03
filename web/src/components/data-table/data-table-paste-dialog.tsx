'use client'

import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { Copy01Icon } from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import { parse as parseCsv } from 'csv-parse/browser/esm/sync'

import { isObject } from './utils'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { Stack, Text } from '@/components/primitives'

interface DataTablePasteDialogProps {
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  trigger?: React.ReactElement
  open?: boolean
  onOpenChange?: (open: boolean) => void
}

export function DataTablePasteDialog({
  onImportCSV,
  onImportJSON,
  trigger,
  open: externalOpen,
  onOpenChange: setExternalOpen,
}: DataTablePasteDialogProps) {
  const [internalOpen, setInternalOpen] = React.useState(false)
  const open = externalOpen ?? internalOpen
  const setOpen = setExternalOpen ?? setInternalOpen

  const [value, setValue] = React.useState('')

  const handleImport = () => {
    if (!value.trim()) {
      toast.error('Please paste some data first.')
      return
    }

    try {
      const trimmed = value.trim()
      // Try JSON first
      if (
        (trimmed.startsWith('{') || trimmed.startsWith('[')) &&
        onImportJSON
      ) {
        const parsed: unknown = JSON.parse(trimmed)
        if (Array.isArray(parsed) && parsed.every(isObject)) {
          onImportJSON(parsed)
          toast.success('JSON data imported successfully.')
          setOpen(false)
          setValue('')
          return
        }
      }

      // Try CSV
      if (onImportCSV) {
        const parsed = parseCsv(trimmed, {
          columns: true,
          skip_empty_lines: true,
          cast: true,
        })
        if (Array.isArray(parsed) && parsed.every(isObject)) {
          onImportCSV(parsed)
          toast.success('CSV data imported successfully.')
          setOpen(false)
          setValue('')
          return
        }
      }

      toast.error('Could not parse the pasted data. Please check the format.')
    } catch (err) {
      console.error(err)
      toast.error(err instanceof Error ? err.message : 'Failed to parse data.')
    }
  }

  const defaultTrigger = (
    <Button variant="outline" size="sm">
      <HugeiconsIcon icon={Copy01Icon} className="size-4" />
      <span>Paste</span>
    </Button>
  )

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={trigger || defaultTrigger} />
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Paste Data</DialogTitle>
          <DialogDescription>
            Paste your data in CSV or JSON format below. Ensure the first row of
            CSV contains headers that match the table columns.
          </DialogDescription>
        </DialogHeader>
        <Stack gap={4} py={4}>
          <Textarea
            placeholder="Paste CSV or JSON here..."
            className="min-h-[300px] font-mono text-xs"
            value={value}
            onChange={(e) => setValue(e.target.value)}
          />
          <Text size="xs" muted>
            Tip: You can copy cells from Excel or Google Sheets and paste them
            here as CSV.
          </Text>
        </Stack>
        <DialogFooter>
          <Button variant="outline" onClick={() => setOpen(false)}>
            Cancel
          </Button>
          <Button onClick={handleImport}>Import Data</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
