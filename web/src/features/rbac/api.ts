import { authClient } from '@/lib/clients'
import {
  assignPermissionSetToStaffMutation,
  assignPermissionToRoleMutation,
  assignPermissionToUserMutation,
  assignPermissionToUserSetMutation,
  createPermissionSetMutation,
  deletePermissionSetMutation,
  getAllPermissionSetsOptions,
  getRolePermissionsOptions,
  getStaffPermissionSetsOptions,
  getUserPermissionsOptions,
  getUserSetMembersOptions,
  getUserSetPermissionsOptions,
  unassignPermissionFromRoleMutation,
  unassignPermissionFromUserMutation,
  unassignPermissionFromUserSetMutation,
  unassignPermissionSetFromStaffMutation,
  updatePermissionSetMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const rbacApi = {
  // Permission Sets (User Sets)
  getSetsOptions: () => getAllPermissionSetsOptions({ client: authClient }),
  createSetMutation: () => createPermissionSetMutation({ client: authClient }),
  deleteSetMutation: () => deletePermissionSetMutation({ client: authClient }),
  updateSetMutation: () => updatePermissionSetMutation({ client: authClient }),

  // Set Permissions
  getSetPermissionsOptions: (setId: string) =>
    getUserSetPermissionsOptions({
      client: authClient,
      path: { user_set_id: setId },
    }),
  assignPermissionToSetMutation: () =>
    assignPermissionToUserSetMutation({ client: authClient }),
  unassignPermissionFromSetMutation: () =>
    unassignPermissionFromUserSetMutation({ client: authClient }),

  // Set Members
  getSetMembersOptions: (setId: string) =>
    getUserSetMembersOptions({
      client: authClient,
      path: { permission_set_id: setId },
    }),

  // Role Permissions
  getRolePermissionsOptions: (role: string) =>
    getRolePermissionsOptions({ client: authClient, path: { role_id: role } }),
  assignPermissionToRoleMutation: () =>
    assignPermissionToRoleMutation({ client: authClient }),
  unassignPermissionFromRoleMutation: () =>
    unassignPermissionFromRoleMutation({ client: authClient }),

  // User Permissions (Direct)
  getUserPermissionsOptions: (userId: string) =>
    getUserPermissionsOptions({
      client: authClient,
      path: { user_id: userId },
    }),
  assignPermissionToUserMutation: () =>
    assignPermissionToUserMutation({ client: authClient }),
  unassignPermissionFromUserMutation: () =>
    unassignPermissionFromUserMutation({ client: authClient }),

  // Staff Permission Sets
  getStaffPermissionSetsOptions: (staffId: string) =>
    getStaffPermissionSetsOptions({
      client: authClient,
      path: { staff_id: staffId },
    }),
  assignSetToStaffMutation: () =>
    assignPermissionSetToStaffMutation({ client: authClient }),
  unassignSetFromStaffMutation: () =>
    unassignPermissionSetFromStaffMutation({ client: authClient }),
}
