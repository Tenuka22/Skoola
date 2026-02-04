import { createFileRoute } from '@tanstack/react-router';
import { GuestGuard } from '@/features/auth/components/guest-guard';

export const Route = createFileRoute('/(auth)/register')({
  component: () => {
    return (
      <GuestGuard>
        <div>
          <h1>Register Page</h1>
          {/* Register form will go here */}
        </div>
      </GuestGuard>
    );
  },
});
