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
    <Box className="flex min-h-screen w-full items-center justify-center p-4">
      <CardPrimitive className="p-0 w-full max-w-sm">
        <CardHeader>
          <CardTitle>
            <Heading size="h3">{title}</Heading>
          </CardTitle>
          {description && (
            <CardDescription>
              <Text size="sm" muted>
                {description}
              </Text>
            </CardDescription>
          )}
        </CardHeader>
        <CardContent>{children}</CardContent>
      </CardPrimitive>
    </Box>
  )
}
