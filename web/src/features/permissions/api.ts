import { authClient } from '@/lib/clients'
import {
  getPermissions9C8839E73223Cb930255A2882A4B0Db4 as getPermissions,
  putPermissions0C5E2C69F1Ce8F3Fb90Ed62D4339Ab5E as updatePermissionApi,
  deletePermissions0C5E2C69F1Ce8F3Fb90Ed62D4339Ab5E as deletePermissionApi,
  postPermissions9C8839E73223Cb930255A2882A4B0Db4 as createPermissionApi,
  // New permission set related APIs
  getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44 as getPermissionSetsApi,
  postPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44 as createPermissionSetApi,
  putPermissionSets9F945C97A8E86681C452E5Cc961Ebc33 as updatePermissionSetApi,
  deletePermissionSets9F945C97A8E86681C452E5Cc961Ebc33 as deletePermissionSetApi,
  postPermissionSetsE88249A62Acbe1Edff95479F9E23B8F3 as assignPermissionToPermissionSetApi,
  deletePermissionSetsE88249A62Acbe1Edff95479F9E23B8F3 as unassignPermissionFromPermissionSetApi,
  postStaff524Cd96166B0B1868B53A942A4154443 as assignPermissionSetToStaffApi,
  deleteStaff524Cd96166B0B1868B53A942A4154443 as unassignPermissionSetFromStaffApi,
  getStaffE1362B25169Eeb0Bc1A99Fcbf3E97Eb2 as getStaffPermissionSetsApi,
  getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92C as getUsersPermissionsApi,
  postUsers069Bc83C67Aeddbeed75C9632Ba56B82 as assignPermissionToUserApi,
  deleteUsers069Bc83C67Aeddbeed75C9632Ba56B82 as unassignPermissionFromUserApi,
} from '@/lib/api/sdk.gen'
import type { PermissionSet } from './types'
import type { Permission } from '@/lib/api/types.gen'
import { PermissionEnum, PermissionSeverity } from '@/lib/api/types.gen'

export const fetchPermissions = async () => {
  const response = await getPermissions({
    client: authClient,
    query: { limit: 100 },
  })
  return (response.data as any)?.data as Permission[]
}

export const createPermission = async (name: PermissionEnum, description: string, safetyLevel: PermissionSeverity) => {
  return createPermissionApi({
    client: authClient,
    body: { name, description, safety_level: safetyLevel },
  })
}

export const updatePermission = async (permissionId: number, name?: PermissionEnum, description?: string, safetyLevel?: PermissionSeverity) => {
  return updatePermissionApi({
    client: authClient,
    path: { permission_id: permissionId },
    body: { name, description, safety_level: safetyLevel },
  })
}

export const deletePermission = async (permissionId: number) => {
  return deletePermissionApi({
    client: authClient,
    path: { permission_id: permissionId },
  })
}


export const fetchPermissionSets = async () => {
  const response = await getPermissionSetsApi({
    client: authClient,
  })
  return (response.data as any)?.data as PermissionSet[]
}

export const createPermissionSet = async (name: string, description: string) => {
  return createPermissionSetApi({
    client: authClient,
    body: { name, description },
  })
}

export const updatePermissionSet = async (setId: string, name?: string, description?: string) => {
  return updatePermissionSetApi({
    client: authClient,
    path: { permission_set_id: setId },
    body: { name, description },
  })
}

export const deletePermissionSet = async (setId: string) => {
  return deletePermissionSetApi({
    client: authClient,
    path: { permission_set_id: setId },
  })
}

export const assignPermissionToPermissionSet = async (setId: string, permissionId: number) => {
  return assignPermissionToPermissionSetApi({
    client: authClient,
    path: { permission_set_id: setId, permission_id: permissionId },
  })
}

export const unassignPermissionFromPermissionSet = async (setId: string, permissionId: number) => {
  return unassignPermissionFromPermissionSetApi({
    client: authClient,
    path: { permission_set_id: setId, permission_id: permissionId },
  })
}

// Staff role management - now uses permission sets
export const assignPermissionSetToStaff = async (staffId: string, setId: string) => {
  return assignPermissionSetToStaffApi({
    client: authClient,
    path: { staff_id: staffId, set_id: setId },
  })
}

export const unassignPermissionSetFromStaff = async (staffId: string, setId: string) => {
  return unassignPermissionSetFromStaffApi({
    client: authClient,
    path: { staff_id: staffId, set_id: setId },
  })
}

export const getStaffPermissionSets = async (staffId: string) => {
  const response = await getStaffPermissionSetsApi({
    client: authClient,
    path: { staff_id: staffId },
  })
  return response.data as PermissionSet[]
}

// User role management - using PermissionSet based APIs.
// The following functions are commented out as there are no direct API endpoints for user permission sets in the generated SDK.
// This functionality needs to be re-evaluated or re-implemented if required.
/*
export const assignPermissionSetToUser = async (userId: string, setId: string) => {
  // Assuming a similar API for users, might need adjustment if different
  return assignPermissionSetToRoleApi({
    client: authClient,
    path: { user_id: userId }, // Assuming user_id for users endpoint
    body: { set_id: setId },
  })
}

export const unassignPermissionSetFromUser = async (userId: string, setId: string) => {
  // Assuming a similar API for users, might need adjustment if different
  return unassignPermissionSetToRoleApi({
    client: authClient,
    path: { user_id: userId, set_id: setId }, // Assuming user_id for users endpoint
  })
}

export const getUserPermissionSets = async (userId: string) => {
  const response = await getPermissionSetsByRoleApi({
    client: authClient,
    path: { user_id: userId }, // Assuming user_id for users endpoint
  })
  return response.data as PermissionSet[]
}
*/

export const fetchUserPermissions = async (userId: string) => {
  const response = await getUsersPermissionsApi({
    client: authClient,
    path: { user_id: userId },
  })
  return (response.data as any)?.data as Permission[]
}

export const assignPermissionToUser = async (userId: string, permissionId: number) => {
  return assignPermissionToUserApi({
    client: authClient,
    path: { user_id: userId, permission_id: permissionId },
  })
}

export const unassignPermissionFromUser = async (userId: string, permissionId: number) => {
  return unassignPermissionFromUserApi({
    client: authClient,
    path: { user_id: userId, permission_id: permissionId },
  })
}
