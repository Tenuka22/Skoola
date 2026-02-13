import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { useUsersStore } from '../store'
import { authClient } from '@/lib/clients'
import {
  deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  getUsers06Bdcf95Aafda840B1D04322636De293QueryKey,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaQueryKey,
  patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation,
} from '@/lib/api/@tanstack/react-query.gen'

export function useUserMutations() {
  const queryClient = useQueryClient()
  const {
    setUserToDelete,
    setIsBulkDeleteOpen,
    setIsBulkEditOpen,
    setIsCreateUserOpen,
    setUserToLock,
    setUserToEdit,
  } = useUsersStore()

  const invalidateUsers = () => {
    queryClient.invalidateQueries({
      predicate: (query) => {
        const key = query.queryKey[0] as any
        return key?._id === getUsers06Bdcf95Aafda840B1D04322636De293QueryKey()
      },
    })
    queryClient.invalidateQueries({
      predicate: (query) => {
        const key = query.queryKey[0] as any
        return (
          key?._id === getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaQueryKey()
        )
      },
    })
  }

  const deleteUser = useMutation({
    ...deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User deleted')
      invalidateUsers()
      setUserToDelete(null)
    },
  })

  const bulkDeleteUsers = useMutation({
    ...deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Users deleted')
      invalidateUsers()
      setIsBulkDeleteOpen(false)
    },
  })

  const updateUser = useMutation({
    ...patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User updated')
      invalidateUsers()
      setUserToEdit(null)
      setUserToLock(null)
    },
  })

  const bulkUpdateUsers = useMutation({
    ...patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Batch update complete')
      invalidateUsers()
      setIsBulkEditOpen(false)
    },
  })

  const createUser = useMutation({
    ...postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User created successfully')
      invalidateUsers()
      setIsCreateUserOpen(false)
    },
    onError: (error: any) => {
      toast.error(error.message || 'Failed to create user')
    },
  })

  return {
    deleteUser,
    bulkDeleteUsers,
    updateUser,
    bulkUpdateUsers,
    createUser,
  }
}
