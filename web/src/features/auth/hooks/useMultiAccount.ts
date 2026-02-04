import { useCallback } from 'react';
import { useLocalStorage } from 'react-use'; // Assuming react-use is available for local storage

// Define a type for an individual account token
interface AuthToken {
  token: string;
  userId: string;
  // Add other relevant user info if needed
}



const LOCAL_STORAGE_KEY = 'skoola-multi-auth-tokens';
const ACTIVE_ACCOUNT_KEY = 'skoola-active-account';

export const useMultiAccount = () => {
  const [storedTokens, setStoredTokens] = useLocalStorage<AuthToken[]>(LOCAL_STORAGE_KEY, []);
  const [activeAccountUserId, setActiveAccountUserId] = useLocalStorage<string>(ACTIVE_ACCOUNT_KEY, '');

  const activeToken = storedTokens?.find(token => token.userId === activeAccountUserId) || null;

  // Add a new account to the list
  const addAccount = useCallback((newToken: AuthToken) => {
    if (!storedTokens?.some(token => token.userId === newToken.userId)) {
      const updatedTokens = [...(storedTokens || []), newToken];
      setStoredTokens(updatedTokens);
      // Automatically switch to the newly added account
      setActiveAccountUserId(newToken.userId);
    }
  }, [storedTokens, setStoredTokens, setActiveAccountUserId]);

  // Remove an account from the list
  const removeAccount = useCallback((userId: string) => {
    const updatedTokens = storedTokens?.filter(token => token.userId !== userId) || [];
    setStoredTokens(updatedTokens);
    if (activeAccountUserId === userId) {
      // If the removed account was active, clear active account
      setActiveAccountUserId('');
    }
  }, [storedTokens, setStoredTokens, activeAccountUserId, setActiveAccountUserId]);

  // Switch the active account
  const switchAccount = useCallback((userId: string) => {
    const accountExists = storedTokens?.some(token => token.userId === userId);
    if (accountExists) {
      setActiveAccountUserId(userId);
    } else {
      console.warn(`Attempted to switch to non-existent account: ${userId}`);
    }
  }, [storedTokens, setActiveAccountUserId]);

  // Impersonate user functionality (for Admins)
  const impersonateUser = useCallback((targetUserToken: AuthToken) => {
    // Logic to switch to a target user's context using an admin's token
    // This might involve:
    // 1. Storing the admin's original token temporarily.
    // 2. Setting the targetUserToken as the active token.
    // 3. Providing a way to revert to the admin's original token.
    // For now, a placeholder, as actual implementation depends on API.
    console.log('Admin impersonating user:', targetUserToken.userId);
    addAccount(targetUserToken); // Add target user's token
    switchAccount(targetUserToken.userId); // Switch to target user's context
  }, [addAccount, switchAccount]);

  // Logout from all accounts
  const logoutAll = useCallback(() => {
    setStoredTokens([]);
    setActiveAccountUserId('');
  }, [setStoredTokens, setActiveAccountUserId]);

  return {
    activeToken,
    allTokens: storedTokens || [],
    addAccount,
    removeAccount,
    switchAccount,
    impersonateUser,
    logoutAll,
    // Add any other state or functions that might be useful
  };
};
