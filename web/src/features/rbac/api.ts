import type { GetAllUsersData } from '@/lib/api/types.gen'
import {
  assignPermissionSetToStaffMutation,
  assignPermissionToRoleMutation,
  assignPermissionToUserMutation,
  assignPermissionToUserSetMutation,
  assignRoleToRoleSetMutation,
  createPermissionSetMutation,
  createRoleSetMutation,
  deletePermissionSetMutation,
  deleteRoleSetMutation,
  getAllPermissionSetsOptions,
  getAllRoleSetsOptions,
  getAllStaffOptions,
  getAllUsersOptions,
  getRolePermissionsOptions,
  getRoleSetRolesOptions,
  getStaffPermissionSetsOptions,
  getUserPermissionsOptions,
  getUserSetMembersOptions,
  getUserSetPermissionsOptions,
  unassignPermissionFromRoleMutation,
  unassignPermissionFromUserMutation,
  unassignPermissionFromUserSetMutation,
  unassignPermissionSetFromStaffMutation,
  unassignRoleFromRoleSetMutation,
  updatePermissionSetMutation,
  updateRoleSetMutation,
  updateUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

/**
 * A central object to hold all RBAC-related tanstack-query options and mutation factories.
 * This follows a similar pattern to the generated file, making it easy to use and mock.
 */
export const rbacApi = {
  // User Management
  getAllUsersOptions: (query?: GetAllUsersData['query']) =>
    getAllUsersOptions({ client: authClient, query }),
  updateUserMutation: () => updateUserMutation({ client: authClient }),

  // Staff Management (for linking users to staff)
  getAllStaffOptions: () => getAllStaffOptions({ client: authClient }),

  // Permission Sets (User Sets)
  getSetsOptions: () => getAllPermissionSetsOptions({ client: authClient }),
  createSetMutation: () => createPermissionSetMutation({ client: authClient }),
  deleteSetMutation: () => deletePermissionSetMutation({ client: authClient }),
  updateSetMutation: () => updatePermissionSetMutation({ client: authClient }),

  // Permissions within a Set
  getSetPermissionsOptions: (setId: string) =>
    getUserSetPermissionsOptions({
      client: authClient,
      path: { user_set_id: setId },
    }),
  assignPermissionToSetMutation: () =>
    assignPermissionToUserSetMutation({ client: authClient }),
  unassignPermissionFromSetMutation: () =>
    unassignPermissionFromUserSetMutation({ client: authClient }),

  // Members of a Set
  getSetMembersOptions: (setId: string) =>
    getUserSetMembersOptions({
      client: authClient,
      path: { user_set_id: setId },
    }),

  // Role Sets
  getRoleSetsOptions: () => getAllRoleSetsOptions({ client: authClient }),
  createRoleSetMutation: () => createRoleSetMutation({ client: authClient }),
  deleteRoleSetMutation: () => deleteRoleSetMutation({ client: authClient }),
  updateRoleSetMutation: () => updateRoleSetMutation({ client: authClient }),

  // Roles within a Role Set
  getRoleSetRolesOptions: (roleSetId: string) =>
    getRoleSetRolesOptions({
      client: authClient,
      path: { role_set_id: roleSetId },
    }),
  assignRoleToRoleSetMutation: () =>
    assignRoleToRoleSetMutation({ client: authClient }),
  unassignRoleFromRoleSetMutation: () =>
    unassignRoleFromRoleSetMutation({ client: authClient }),

  // Role Permissions
  getRolePermissionsOptions: (role: string) =>
    getRolePermissionsOptions({ client: authClient, path: { role_id: role } }),
  assignPermissionToRoleMutation: () =>
    assignPermissionToRoleMutation({ client: authClient }),
  unassignPermissionFromRoleMutation: () =>
    unassignPermissionFromRoleMutation({ client: authClient }),

  // Direct User Permissions
  getUserPermissionsOptions: (userId: string) =>
    getUserPermissionsOptions({
      client: authClient,
      path: { user_id: userId },
    }),
  assignPermissionToUserMutation: () =>
    assignPermissionToUserMutation({ client: authClient }),
  unassignPermissionFromUserMutation: () =>
    unassignPermissionFromUserMutation({ client: authClient }),

  // Linking Permission Sets to Staff
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
