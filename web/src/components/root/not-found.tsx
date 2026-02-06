import { Link } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'

export function NotFound() {
  return (
    <div className="flex min-h-screen w-full items-center justify-center p-4 bg-muted/40">
      <Empty className="max-w-sm bg-background border-border">
        <EmptyHeader>
          <EmptyTitle>Page Not Found</EmptyTitle>
          <EmptyDescription>
            The page you are looking for does not exist or you may need to log in.
          </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button render={
            <Link to="/login">Go to Login</Link>
          } className="w-full" />
        </EmptyContent>
      </Empty>
    </div>
  )
}