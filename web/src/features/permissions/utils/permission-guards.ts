import type { PermissionSet } from '../types'
import type { PermissionsViewMode } from '../store'

export const isPermissionSetArray = (
  data: unknown,
): data is Array<PermissionSet> => {
  // Basic runtime check: ensure it's an array and each item has at least an 'id' property
  // More comprehensive checks could be added here if needed, e.g., checking other properties
  return (
    Array.isArray(data) &&
    data.every(
      (item) =>
        typeof item === 'object' &&
        item !== null &&
        'id' in item &&
        'name' in item,
    )
  )
}

export const isPermissionsViewMode = (
  value: string,
): value is PermissionsViewMode => {
  return value === 'permissions' || value === 'permission-sets'
}

