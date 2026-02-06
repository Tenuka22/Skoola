import { client } from '@/lib/api/client.gen'
import type { PaginatedUserResponse, UserStatsResponse, ClientResponseWrapper } from './types'

export async function getUsers({
  page,
  limit,
  search,
  is_verified,
  auth_method,
  sort_by,
  sort_order,
}: {
  page: number
  limit: number
  search?: string
  is_verified?: boolean
  auth_method?: string
  sort_by?: string
  sort_order?: 'asc' | 'desc'
}) {
  const response = (await client.get<PaginatedUserResponse>({
    url: '/users',
    query: { page, limit, search, is_verified, auth_method, sort_by, sort_order },
  })) as ClientResponseWrapper<PaginatedUserResponse>

  if (response.error) throw new Error(response.error as string)
  if (!response.data) throw new Error('No data received.')
  return response.data
}

export async function getUserStats() {
  const response = (await client.get<UserStatsResponse>({
    url: '/users/stats',
  })) as ClientResponseWrapper<UserStatsResponse>

  if (response.error) throw new Error(response.error as string)
  if (!response.data) throw new Error('No stats received.')
  return response.data
}

export async function deleteUser(userId: string) {
  const response = await client.delete({ url: `/users/${userId}` })
  if (response.error) throw new Error(response.error as string)
  return response.data
}

export async function bulkDeleteUsers(userIds: string[]) {
  const response = await client.delete({
    url: '/users/bulk',
    body: { user_ids: userIds },
  })
  if (response.error) throw new Error(response.error as string)
  return response.data
}

export async function updateUser(userId: string, data: { is_verified?: boolean; is_locked?: boolean }) {
  const response = await client.patch({
    url: `/users/${userId}`,
    body: data,
  })
  if (response.error) throw new Error(response.error as string)
  return response.data
}

export async function bulkUpdateUsers(userIds: string[], data: { is_verified?: boolean }) {
  const response = await client.patch({
    url: '/users/bulk',
    body: { user_ids: userIds, ...data },
  })
  if (response.error) throw new Error(response.error as string)
  return response.data
}