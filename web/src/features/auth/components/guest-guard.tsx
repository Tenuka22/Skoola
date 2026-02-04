import React, { useEffect } from 'react';
import { useNavigate } from '@tanstack/react-router';
import { useMultiAccount } from '@/features/auth/hooks/useMultiAccount'; // Adjust import path as needed

interface GuestGuardProps {
  children: React.ReactNode;
}

export const GuestGuard = ({ children }: GuestGuardProps) => {
  const navigate = useNavigate();
  const { activeToken } = useMultiAccount();

  useEffect(() => {
    if (activeToken) {
      // User is logged in, redirect to dashboard
      navigate({ to: '/_admin/dashboard' }); // Assuming a dashboard route under /_admin
    }
  }, [activeToken, navigate]);

  if (activeToken) {
    // Optionally render a loading spinner or null while redirecting
    return null;
  }

  return <>{children}</>;
};
