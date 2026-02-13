import type { AuthStorage } from '@/lib/auth/session'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { cn } from '@/lib/utils'

interface ActiveSessionsProps {
  authStorage: AuthStorage | null
}

export function ActiveSessions({ authStorage }: ActiveSessionsProps) {
  if (
    !authStorage?.sessions ||
    Object.keys(authStorage.sessions).length === 0
  ) {
    return null
  }

  return (
    <div className="flex flex-col gap-2">
      <span className="text-sm text-muted-foreground">
        Already logged in as
      </span>
      <div className="flex -space-x-2">
        {Object.entries(authStorage.sessions)
          .sort(([keyA], [keyB]) => {
            if (keyA === authStorage.activeUserId) return -1
            if (keyB === authStorage.activeUserId) return 1
            return 0
          })
          .map(([key, value]) => (
            <TooltipProvider key={key}>
              <Tooltip>
                <TooltipTrigger>
                  <div className="relative">
                    <Avatar className="size-9">
                      <AvatarImage src={undefined} alt={value.user.email} />
                      <AvatarFallback
                        className={cn(
                          'uppercase',
                          key === authStorage.activeUserId &&
                            'bg-primary text-primary-foreground',
                        )}
                      >
                        {String(value.user.email).substring(0, 2).toUpperCase()}
                      </AvatarFallback>
                    </Avatar>
                    {key === authStorage.activeUserId && (
                      <div className="absolute -bottom-0.5 -right-0.5 h-3 w-3 rounded-full border-2 border-background bg-green-500" />
                    )}
                  </div>
                </TooltipTrigger>
                <TooltipContent>
                  <p className="text-sm">{value.user.email}</p>
                  {key === authStorage.activeUserId && (
                    <p className="text-xs text-muted-foreground">Active</p>
                  )}
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          ))}
      </div>
    </div>
  )
}
