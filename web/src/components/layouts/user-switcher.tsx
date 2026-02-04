
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import * as Icons from '@hugeicons/react';
import { useMultiAccount } from '@/features/auth/hooks/useMultiAccount';

export const UserSwitcher = () => {
  const { activeToken, allTokens, switchAccount } = useMultiAccount();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild={true}>
        <Button variant="outline" className="flex items-center gap-2">
          <span>{activeToken ? activeToken.userId : 'Select Account'}</span>
          <Icons.ChevronsUpDown className="h-4 w-4 opacity-50" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel>Switch Account</DropdownMenuLabel>
        <DropdownMenuSeparator />
        {allTokens.length > 0 ? (
          allTokens.map((token) => (
            <DropdownMenuItem key={token.userId} onClick={() => switchAccount(token.userId)}>
              {token.userId} {activeToken?.userId === token.userId && ' (Active)'}
            </DropdownMenuItem>
          ))
        ) : (
          <DropdownMenuItem disabled>No other accounts</DropdownMenuItem>
        )}
        <DropdownMenuSeparator />
        {/* Future: Add option to add a new account or logout */}
        <DropdownMenuItem>Add New Account</DropdownMenuItem>
        <DropdownMenuItem>Logout</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
