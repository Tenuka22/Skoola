import { createFileRoute } from '@tanstack/react-router';
import { GuestGuard } from '@/features/auth/components/guest-guard';

export const Route = createFileRoute('/(auth)/login')({
  component: () => {
    return (
      <GuestGuard>
        <div>
          <h1>Login Page</h1>
          {/* Login form will go here */}
        </div>
      </GuestGuard>
    );
  },
});
