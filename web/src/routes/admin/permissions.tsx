import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { toast } from 'sonner'

import { PermissionsHeader } from '../../features/permissions/components/permissions-header'
import { PermissionsToolbar } from '../../features/permissions/components/permissions-toolbar'
import { PermissionsFilters } from '../../features/permissions/components/permissions-filters'
import { PermissionsListContainer } from '../../features/permissions/components/permissions-list-container'
import { PermissionModals } from '../../features/permissions/components/permission-modals'
import { usePermissionsStore } from '../../features/permissions/store'
import { isPermissionSetArray } from '../../features/permissions/utils/permission-guards'
import { authClient } from '@/lib/clients'
import {
  deletePermissions0C5E2C69F1Ce8F3Fb90Ed62D4339Ab5eMutation as deletePermissionMutation,
  deletePermissionSets9F945C97A8E86681C452E5Cc961Ebc33Mutation as deletePermissionSetMutation,
  getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44Options as getPermissionSetsOptions,
  getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44QueryKey as getPermissionSetsQueryKey,
  getPermissions9C8839E73223Cb930255A2882A4B0Db4Options as getPermissionsOptions,
  getPermissions9C8839E73223Cb930255A2882A4B0Db4QueryKey as getPermissionsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'

export const Route = createFileRoute('/admin/permissions')({
  component: PermissionsPage,
})

function PermissionsPage() {
  const store = usePermissionsStore()
  const {
    permissionsPage,
    permissionsSearch,
    permissionsSorting,
    setPermissionToDelete,
    setPermissionSetToDelete,
  } = store

  const limit = 10
  const sortBy = permissionsSorting[0]?.id || 'name'
  const sortOrder = permissionsSorting[0]?.desc ? 'desc' : 'asc'

  // Fetch permissions
  const permissionsQuery = useQuery({
    ...getPermissionsOptions({
      client: authClient,
      query: {
        page: permissionsPage,
        limit: limit,
        search: permissionsSearch,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })

  // Fetch permission sets
  const permissionSetsQuery = useQuery({
    ...getPermissionSetsOptions({
      client: authClient,
    }),
    placeholderData: keepPreviousData,
    select: (data) => (isPermissionSetArray(data) ? data : []),
  })

  const queryClient = useQueryClient()

  const invalidatePermissions = () => {
    queryClient.invalidateQueries({ queryKey: getPermissionsQueryKey() })
  }

  const invalidatePermissionSets = () => {
    queryClient.invalidateQueries({ queryKey: getPermissionSetsQueryKey() })
  }

  const deletePermission = useMutation({
    ...deletePermissionMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Permission deleted successfully.')
      invalidatePermissions()
      setPermissionToDelete(null)
    },
    onError: (error) => {
      toast.error(`Failed to delete permission: ${error.message}`)
    },
  })

  const deletePermissionSet = useMutation({
    ...deletePermissionSetMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Permission set deleted successfully.')
      invalidatePermissionSets()
      setPermissionSetToDelete(null)
    },
    onError: (error) => {
      toast.error(`Failed to delete permission set: ${error.message}`)
    },
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <PermissionsHeader />
      <PermissionsToolbar />
      <PermissionsFilters />
      <PermissionsListContainer
        permissionsQuery={permissionsQuery}
        permissionSetsQuery={permissionSetsQuery}
        limit={limit}
      />

      <PermissionModals
        onPermissionDeleteConfirm={(id) =>
          deletePermission.mutate({ path: { permission_id: id } })
        }
        onPermissionSetDeleteConfirm={(id) =>
          deletePermissionSet.mutate({ path: { permission_set_id: id } })
        }
        allPermissions={permissionsQuery.data?.data || []}
      />
    </div>
  )
}
