import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'
import { _ProfileCreate_ } from '@/lib/api/sdk.gen'
import { staffGetAllQueryKey } from '@/lib/api/@tanstack/react-query.gen'

export interface ImportStaffRecord {
  name?: string
  email?: string
  phone?: string
  dob?: string
  gender?: string
  employee_id?: string
  staff_type?: string
  employment_status?: string
  address?: string
  nic?: string
  [key: string]: unknown
}

export const useBulkImportStaff = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (staffMembers: Array<ImportStaffRecord>) => {
      const results: {
        success: number
        failed: number
        errors: Array<string>
      } = {
        success: 0,
        failed: 0,
        errors: [],
      }

      for (const staff of staffMembers) {
        if (!staff.name || !staff.employee_id || !staff.staff_type) {
          results.failed++
          results.errors.push(
            `Missing required fields (name, employee_id, staff_type) for record: ${JSON.stringify(staff)}`,
          )
          continue
        }

        try {
          await _ProfileCreate_({
            client: authClient,
            body: {
              name: staff.name,
              created_at: new Date().toISOString(),
              id: crypto.randomUUID(),
              updated_at: new Date().toISOString(),
            },
            throwOnError: true,
          })
          results.success++
        } catch (error: unknown) {
          results.failed++
          const message =
            error instanceof Error ? error.message : 'Unknown error'
          results.errors.push(`Failed to register ${staff.name}: ${message}`)
        }
      }

      return results
    },
    onSuccess: (results) => {
      if (results.success > 0) {
        toast.success(`Successfully imported ${results.success} staff members.`)
        queryClient.invalidateQueries({
          queryKey: staffGetAllQueryKey(),
        })
      }

      if (results.failed > 0) {
        toast.error(
          `Failed to import ${results.failed} staff members. Check console for details.`,
        )
        console.error('Import errors:', results.errors)
      }
    },
    onError: (error: Error) => {
      const message = error instanceof Error ? error.message : 'Unknown error'
      toast.error(`Import failed: ${message}`)
    },
  })
}
