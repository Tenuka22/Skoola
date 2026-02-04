import { createFileRoute } from '@tanstack/react-router';
import { GuestGuard } from '@/features/auth/components/guest-guard';

export const Route = createFileRoute('/(auth)/forgot-password')({
  component: () => {
    return (
      <GuestGuard>
        <div>
          <h1>Forgot Password Page</h1>
          {/* Forgot password form will go here */}
        </div>
      </GuestGuard>
    );
  },
});
