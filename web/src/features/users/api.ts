import type { GetAllUsersData, UpdateUserRequest } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import {
  bulkDeleteUsers as bulkDeleteUsersApi,
  bulkUpdateUsers as bulkUpdateUsersApi,
  deleteUser as deleteUserApi,
  getAllUsers,
  getUserStatistics,
  updateUser as updateUserApi,
} from '@/lib/api/sdk.gen'

export async function getUsers(query: GetAllUsersData['query']) {
  const { data } = await getAllUsers({
    client: authClient,
    query,
    throwOnError: true,
  })
  return data
}

export async function getUserStats() {
  const { data } = await getUserStatistics({
    client: authClient,
    throwOnError: true,
  })
  return data
}

export async function deleteUser(userId: string) {
  const { data } = await deleteUserApi({
    client: authClient,
    path: { user_id: userId },
    throwOnError: true,
  })
  return data
}

export async function bulkDeleteUsers(userIds: Array<string>) {
  const { data } = await bulkDeleteUsersApi({
    client: authClient,
    body: { userIds: userIds },
    throwOnError: true,
  })
  return data
}

export async function updateUser(userId: string, data: UpdateUserRequest) {
  const { data: response_data } = await updateUserApi({
    client: authClient,
    path: { user_id: userId },
    body: data,
    throwOnError: true,
  })
  return response_data
}

export async function bulkUpdateUsers(
  userIds: Array<string>,
  data: { is_verified?: boolean },
) {
  const { data: response_data } = await bulkUpdateUsersApi({
    client: authClient,
    body: { user_ids: userIds, ...data },
    throwOnError: true,
  })
  return response_data
}
