import { Link } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'
import { Box, Heading, Text } from '@/components/primitives'

export function NotFound() {
  return (
    <Box className="flex min-h-screen w-full items-center justify-center p-4 bg-muted/40">
      <Empty className="max-w-sm bg-background border-border">
        <EmptyHeader>
          <EmptyTitle>
            <Heading size="h4">Page Not Found</Heading>
          </EmptyTitle>
          <EmptyDescription>
            <Text size="sm" muted>
              The page you are looking for does not exist or you may need to log
              in.
            </Text>
          </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button render={<Link to="/">Go to Home</Link>} className="w-full" />
        </EmptyContent>
      </Empty>
    </Box>
  )
}
