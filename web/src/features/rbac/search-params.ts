import { parseAsBoolean, parseAsString, useQueryState } from 'nuqs'

export const useRBACSearchParams = () => {
  const [activeTab, setActiveTab] = useQueryState(
    'tab',
    parseAsString.withDefault('users'),
  )

  const [selectedUserId, setSelectedUserId] = useQueryState(
    'user_id',
    parseAsString,
  )
  const [selectedRoleId, setSelectedRoleId] = useQueryState(
    'role_id',
    parseAsString,
  )
  const [selectedPermissionSetId, setSelectedPermissionSetId] = useQueryState(
    'permission_set_id',
    parseAsString,
  )
  const [selectedRoleSetId, setSelectedRoleSetId] = useQueryState(
    'role_set_id',
    parseAsString,
  )

  const [isRoleEditorOpen, setIsRoleEditorOpen] = useQueryState(
    'edit_role',
    parseAsBoolean.withDefault(false),
  )

  return {
    activeTab,
    setActiveTab,
    selectedUserId,
    setSelectedUserId,
    selectedRoleId,
    setSelectedRoleId,
    selectedPermissionSetId,
    setSelectedPermissionSetId,
    selectedRoleSetId,
    setSelectedRoleSetId,
    isRoleEditorOpen,
    setIsRoleEditorOpen,
  }
}
