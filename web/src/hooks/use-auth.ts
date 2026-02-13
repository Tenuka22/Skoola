import { useEffect, useState } from 'react'
import type { Session } from '@/lib/auth/session'
import { getActiveSessionServer } from '@/lib/auth/session'

export const useAuth = () => {
  const [session, setSession] = useState<Session | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    const fetchSession = async () => {
      try {
        const activeSession = await getActiveSessionServer()
        setSession(activeSession)
      } catch (error) {
        console.error('Failed to fetch session:', error)
      } finally {
        setIsLoading(false)
      }
    }

    fetchSession()
  }, [])

  return {
    session,
    user: session?.user,
    isLoading,
    isAuthenticated: !!session,
  }
}
