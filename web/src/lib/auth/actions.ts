import { createServerFn } from '@tanstack/react-start'
import { z } from 'zod'
import {
  getProfileC838C8E7Da73Bfc08645A117E4Df91F3 as getProfileApi,
  postAuthLogin9E9Be264D609C0E1A535693Ba4C389Aa as loginApi,
  postAuthLogout5D5C18E2301F7F66A8222C30Cd9230A0 as logoutApi,
  postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9 as signUpApi,
} from '../api/sdk.gen'
import { authClient, publicClient } from '../clients'
import { createClient } from '../api/client/index'
import { env } from '../env'
import {
  SessionSchema,
  addSessionServer,
  clearAuthServer,
  getActiveSessionServer,
} from './session'

const loginSchema = z.object({
  email: z.email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
})

const signUpSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
})

export const loginFn = createServerFn({ method: 'POST' })
  .inputValidator((data: z.infer<typeof loginSchema>) =>
    loginSchema.parse(data),
  )
  .handler(async ({ data }) => {
    try {
      const loginResponse = await loginApi({
        client: publicClient,
        body: {
          email: data.email,
          password: data.password,
        },
      })

      if (loginResponse.data?.token) {
        const tempAuthClient = createClient({
          baseUrl: env.VITE_API_URL,
          headers: {
            Authorization: `Bearer ${loginResponse.data.token}`,
          },
        })
        const userProfileResponse = await getProfileApi({
          client: tempAuthClient,
        })

        if (userProfileResponse.data) {
          const newSession = SessionSchema.parse({
            tokens: loginResponse.data,
            user: {
              id: userProfileResponse.data.id,
              email: userProfileResponse.data.email,
              is_verified: userProfileResponse.data.is_verified,
              roles: userProfileResponse.data.roles,
              created_at: userProfileResponse.data.created_at,
              updated_at: userProfileResponse.data.updated_at,
            },
          })
          await addSessionServer({ data: newSession })
          return { success: true }
        } else {
          return {
            success: false,
            error: 'Failed to retrieve user profile after login.',
          }
        }
      } else {
        return { success: false, error: 'Login failed: No token received' }
      }
    } catch (error: unknown) {
      console.error('Login error:', error)
      return {
        success: false,
        error:
          (error instanceof Error && error.message) ||
          'Login failed, please check your credentials.',
      }
    }
  })

export const signUpFn = createServerFn({ method: 'POST' })
  .inputValidator((data: z.infer<typeof signUpSchema>) =>
    signUpSchema.parse(data),
  )
  .handler(async ({ data }) => {
    try {
      const signUpResponse = await signUpApi({
        client: publicClient,
        body: {
          email: data.email,
          password: data.password,
        },
      })
      console.log('Sign Up API response:', signUpResponse)
      if (signUpResponse.data) {
        return { success: true }
      } else {
        return { success: false, error: 'Sign up failed: No data received' }
      }
    } catch (error: unknown) {
      console.error('Sign up error:', error)
      return {
        success: false,
        error:
          (error instanceof Error && error.message) ||
          'Sign up failed, please try again.',
      }
    }
  })

export const logoutFn = createServerFn({ method: 'POST' }).handler(async () => {
  try {
    const session = await getActiveSessionServer()
    if (session?.tokens?.refresh_token) {
      await logoutApi({
        client: authClient,
        body: {
          refresh_token: session.tokens.refresh_token,
        },
      })
    }
  } catch (error) {
    console.error('Logout API error:', error)
  } finally {
    await clearAuthServer()
  }
})

export const reloginNeeded = createServerFn({ method: 'GET' }).handler(async () => {
  await clearAuthServer()
  return { redirect: '/login' }
})