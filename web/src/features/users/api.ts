import type {
  GetUsers06Bdcf95Aafda840B1D04322636De293Data,
  UpdateUserRequest,
} from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import {
  deleteUsers5D3C91131F7D9Efc5999C92Dbfac75Da,
  deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8,
  getUsers06Bdcf95Aafda840B1D04322636De293,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9Fba,
  patchUsers5D3C91131F7D9Efc5999C92Dbfac75Da,
  patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8,
} from '@/lib/api/sdk.gen'

export async function getUsers(
  query: GetUsers06Bdcf95Aafda840B1D04322636De293Data['query'],
) {
  const { data } = await getUsers06Bdcf95Aafda840B1D04322636De293({
    client: authClient,
    query,
    throwOnError: true,
  })
  return data
}

export async function getUserStats() {
  const { data } = await getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9Fba({
    client: authClient,
    throwOnError: true,
  })
  return data
}

export async function deleteUser(userId: string) {
  const { data } = await deleteUsers5D3C91131F7D9Efc5999C92Dbfac75Da({
    client: authClient,
    path: { user_id: userId },
    throwOnError: true,
  })
  return data
}

export async function bulkDeleteUsers(userIds: Array<string>) {
  const { data } = await deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8({
    client: authClient,
    body: { user_ids: userIds },
    throwOnError: true,
  })
  return data
}

export async function updateUser(userId: string, data: UpdateUserRequest) {
  const { data: response_data } =
    await patchUsers5D3C91131F7D9Efc5999C92Dbfac75Da({
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
  const { data: response_data } =
    await patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8({
      client: authClient,
      body: { user_ids: userIds, ...data },
      throwOnError: true,
    })
  return response_data
}
