import * as React from 'react'
import {
  CardContent,
  CardDescription,
  CardHeader,
  Card as CardPrimitive,
  CardTitle,
} from '@/components/ui/card'
import { Box, Heading, Text } from '@/components/primitives'

interface AuthLayoutProps {
  title: string
  description?: string
  children: React.ReactNode
}

export function AuthLayout({ title, description, children }: AuthLayoutProps) {
  return (
    <Box className="flex min-h-screen w-full items-center justify-center p-6 bg-background relative overflow-hidden">
      {/* Subtle radial glow */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,var(--tw-gradient-stops))] from-primary/3 via-transparent to-transparent pointer-events-none" />

      <div className="relative z-10 w-full max-w-[380px] animate-in fade-in slide-in-from-bottom-4 duration-500">
        {/* Logo / Brand */}
        <div className="flex flex-col items-center mb-8">
          <div className="size-10 rounded-xl bg-primary flex items-center justify-center mb-4 shadow-lg shadow-primary/20">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              className="size-5 text-primary-foreground"
              stroke="currentColor"
              strokeWidth="2.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
              <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
            </svg>
          </div>
          <Text
            size="sm"
            muted
            className="font-medium tracking-wide uppercase text-[10px] letter-spacing-wider"
          >
            Skoola
          </Text>
        </div>

        <CardPrimitive className="p-0 w-full border-border/50 bg-card shadow-xl shadow-black/3 dark:shadow-black/30 rounded-2xl overflow-hidden">
          <CardHeader className="pb-2 pt-7 px-7 space-y-1.5">
            <CardTitle>
              <Heading size="h4" className="tracking-tight font-semibold">
                {title}
              </Heading>
            </CardTitle>
            {description && (
              <CardDescription>
                <Text size="sm" muted className="leading-relaxed">
                  {description}
                </Text>
              </CardDescription>
            )}
          </CardHeader>
          <CardContent className="px-7 pb-7">{children}</CardContent>
        </CardPrimitive>
      </div>
    </Box>
  )
}
