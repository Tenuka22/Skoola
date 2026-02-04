
import { Button } from '@/components/ui/button';
import * as Icons from '@hugeicons/react';
import { useSidebar } from './sidebar-context';

export const TopNavigation = () => {
  const { toggleSidebar } = useSidebar();

  return (
    <header className="flex items-center justify-between p-4 border-b bg-background sticky top-0 z-40">
      <div className="flex items-center gap-4">
        {/* Mobile sidebar toggle button */}
        <Button
          variant="ghost"
          size="icon"
          className="md:hidden"
          onClick={toggleSidebar}
        >
          <Icons.Menu className="h-6 w-6" />
        </Button>
        <h1 className="text-xl font-semibold">Skoola Admin Dashboard</h1>
      </div>
      <div className="flex items-center gap-4">
        {/* UserSwitcher will go here */}
        {/* User menu/avatar will go here */}
      </div>
    </header>
  );
};
