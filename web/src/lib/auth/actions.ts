import { createServerFn } from '@tanstack/react-start'
import { z } from 'zod'
import { getProfile, loginUser, logoutUser, registerUser } from '../api/sdk.gen'
import { zLoginRequest, zRegisterRequest } from '../api/zod.gen'
import { authClient, publicClient } from '../clients'
import { createClient } from '../api/client/index'
import { env } from '../env'
import {
  SessionSchema,
  addSessionServer,
  clearAuthServer,
  getActiveSessionServer,
} from './session'

const loginSchema = zLoginRequest.extend({
  email: z.email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
})

const signUpSchema = zRegisterRequest.extend({
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
      const loginResponse = await loginUser({
        client: publicClient,
        body: {
          email: data.email,
          password: data.password,
        },
      })

      if (!loginResponse.data) {
        return {
          success: false,
          error: 'Login failed: Please verify the credentiaals.',
        }
      }

      const tempAuthClient = createClient({
        baseUrl: env.VITE_API_URL,
        headers: {
          Authorization: `Bearer ${loginResponse.data.token}`,
        },
      })

      const userProfileResponse = await getProfile({
        client: tempAuthClient,
      })

      if (!userProfileResponse.data) {
        return {
          success: false,
          error: 'Failed to retrieve user profile after login.',
        }
      }

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

      try {
        await addSessionServer({ data: newSession })
      } catch (sessionError: unknown) {
        console.error('Session saving error:', sessionError)
        return {
          success: false,
          error:
            sessionError instanceof Error
              ? `Failed to save session: ${sessionError.message}`
              : 'Failed to save session due to unknown error.',
        }
      }

      return { success: true }
    } catch (error: unknown) {
      console.error('Login error:', error)
      return {
        success: false,
        error:
          error instanceof Error
            ? `Login failed: ${error.message}`
            : 'Login failed due to an unknown error.',
      }
    }
  })
export const signUpFn = createServerFn({ method: 'POST' })
  .inputValidator((data: z.infer<typeof signUpSchema>) =>
    signUpSchema.parse(data),
  )
  .handler(async ({ data }) => {
    try {
      const signUpResponse = await registerUser({
        client: publicClient,
        body: {
          email: data.email,
          password: data.password,
        },
      })

      if (!signUpResponse.data) {
        return { success: false, error: 'Sign up failed: No data received' }
      }

      return { success: true }
    } catch (error: unknown) {
      return {
        success: false,
        error:
          error instanceof Error
            ? `Sign up failed: ${error.message}`
            : 'Sign up failed, please try again.',
      }
    }
  })

export const logoutFn = createServerFn({ method: 'POST' }).handler(async () => {
  try {
    const session = await getActiveSessionServer()
    if (session?.tokens?.refresh_token) {
      await logoutUser({
        client: authClient,
        body: { refresh_token: session.tokens.refresh_token },
      })
    }
  } catch (error: unknown) {
    console.error('Logout API error:', error)
  } finally {
    await clearAuthServer()
  }
})

export const reloginNeeded = createServerFn({ method: 'GET' }).handler(
  async () => {
    await clearAuthServer()
    return { redirect: '/login' }
  },
)
