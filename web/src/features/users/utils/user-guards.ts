type AuthMethod = 'Email' | 'Google' | 'Github'

export const isAuthMethod = (value: string): value is AuthMethod => {
  return value === 'Email' || value === 'Google' || value === 'Github'
}
