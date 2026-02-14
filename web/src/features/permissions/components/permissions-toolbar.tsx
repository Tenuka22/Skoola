import {
  Add01Icon,
  LayoutGridIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { usePermissionsStore } from '../store'
import type { PermissionsViewMode } from '../store'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'

export function PermissionsToolbar() {
  const {
    view,
    setView,
    permissionsSearch,
    setPermissionsSearch,
    permissionSetsSearch,
    setPermissionSetsSearch,
    setIsCreatePermissionOpen,
    setIsCreatePermissionSetOpen,
  } = usePermissionsStore()

  const isPermissions = view === 'permissions'
  const searchValue = isPermissions ? permissionsSearch : permissionSetsSearch
  const setSearch = isPermissions
    ? setPermissionsSearch
    : setPermissionSetsSearch

  return (
    <div className="mb-4 flex flex-col gap-4 px-8 sm:flex-row sm:items-center sm:justify-between">
      <Tabs
        value={view}
        onValueChange={(value) => setView(value as PermissionsViewMode)}
      >
        <TabsList>
          <TabsTrigger value="permissions" className="gap-2">
            <HugeiconsIcon icon={TableIcon} className="size-4" />
            Permissions
          </TabsTrigger>
          <TabsTrigger value="permission-sets" className="gap-2">
            <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
            Permission Sets
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
        <div className="relative flex-1 sm:w-64">
          <InputGroup>
            <InputGroupInput
              value={searchValue}
              onChange={(e) => setSearch(e.target.value)}
              placeholder={`Search ${isPermissions ? 'permissions' : 'sets'}...`}
            />
            <InputGroupAddon>
              <HugeiconsIcon icon={Search01Icon} />
            </InputGroupAddon>
          </InputGroup>
        </div>

        <Button
          size="sm"
          className="gap-2"
          onClick={() =>
            isPermissions
              ? setIsCreatePermissionOpen(true)
              : setIsCreatePermissionSetOpen(true)
          }
        >
          <HugeiconsIcon icon={Add01Icon} className="size-4" />
          Add {isPermissions ? 'Permission' : 'Set'}
        </Button>
      </div>
    </div>
  )
}
