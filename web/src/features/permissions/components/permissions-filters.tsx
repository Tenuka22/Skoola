import { usePermissionsStore } from '../store'
import { Button } from '@/components/ui/button'

export function PermissionsFilters() {
  const {
    view,
    permissionsSearch,
    setPermissionsSearch,
    permissionSetsSearch,
    setPermissionSetsSearch,
  } = usePermissionsStore()

  const isPermissions = view === 'permissions'
  const hasFilters = isPermissions
    ? permissionsSearch !== ''
    : permissionSetsSearch !== ''

  const clearFilters = () => {
    if (isPermissions) {
      setPermissionsSearch('')
    } else {
      setPermissionSetsSearch('')
    }
  }

  if (!hasFilters) return null

  return (
    <div className="mb-4 flex flex-wrap items-center gap-2 px-8">
      <Button
        variant="ghost"
        size="sm"
        className="h-8 px-2 text-xs"
        onClick={clearFilters}
      >
        Clear Filters
      </Button>
    </div>
  )
}
