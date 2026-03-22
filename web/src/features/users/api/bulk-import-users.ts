import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'
import { registerUser } from '@/lib/api/sdk.gen'
import { userGetAllQueryKey } from '@/lib/api/@tanstack/react-query.gen'

export interface ImportUserRecord {
  email?: string
  password?: string
  [key: string]: unknown
}

export const useBulkImportUsers = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (users: Array<ImportUserRecord>) => {
      const results: {
        success: number
        failed: number
        errors: Array<string>
      } = {
        success: 0,
        failed: 0,
        errors: [],
      }

      for (const user of users) {
        if (!user.email || !user.password) {
          results.failed++
          results.errors.push(
            `Missing email or password for record: ${JSON.stringify(user)}`,
          )
          continue
        }

        try {
          await registerUser({
            client: authClient,
            body: {
              email: user.email,
              password: user.password,
            },
            throwOnError: true,
          })
          results.success++
        } catch (error: unknown) {
          results.failed++
          const message =
            error instanceof Error ? error.message : 'Unknown error'
          results.errors.push(`Failed to register ${user.email}: ${message}`)
        }
      }

      return results
    },
    onSuccess: (results) => {
      if (results.success > 0) {
        toast.success(`Successfully imported ${results.success} users.`)
        queryClient.invalidateQueries({
          queryKey: userGetAllQueryKey(),
        })
      }

      if (results.failed > 0) {
        toast.error(
          `Failed to import ${results.failed} users. Check console for details.`,
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
