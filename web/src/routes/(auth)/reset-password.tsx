import { createFileRoute } from '@tanstack/react-router';
import { GuestGuard } from '@/features/auth/components/guest-guard';

export const Route = createFileRoute('/(auth)/reset-password')({
  component: () => {
    return (
      <GuestGuard>
        <div>
          <h1>Reset Password Page</h1>
          {/* Reset password form will go here */}
        </div>
      </GuestGuard>
    );
  },
});
