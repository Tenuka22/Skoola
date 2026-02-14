import type { UserResponse, UserProfileResponse } from '@/lib/api/types.gen'

export const mapUserResponseToUserProfile = (user: UserResponse): UserProfileResponse => {
  return {
    ...user,
    roles: [], // UserResponse does not have roles, so we default to an empty array
  }
}
