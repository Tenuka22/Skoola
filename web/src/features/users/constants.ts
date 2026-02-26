export const USER_AUTH_METHODS = ['google', 'github', 'password'] as const

export type UserAuthMethod = (typeof USER_AUTH_METHODS)[number]
export type UserAuthFilter = UserAuthMethod | 'all'
