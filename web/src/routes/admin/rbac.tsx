import { createFileRoute } from '@tanstack/react-router'

import { useRBACStore } from '../../features/rbac/store'
import { RBACHeader } from '../../features/rbac/components/rbac-header'
import { UsersTab } from '../../features/rbac/components/users-tab'
import { RolesTab } from '../../features/rbac/components/roles-tab'
import { PermissionSetsTab } from '../../features/rbac/components/permission-sets-tab'
import { RoleEditorDialog } from '../../features/rbac/components/role-editor-dialog'
import { TabsContent, Tabs, TabsTrigger, TabsList } from '@/components/ui/tabs'
import { RBACActiveTab } from '@/features/rbac/constants'
import {
  Layers01Icon,
  Shield01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

export const Route = createFileRoute('/admin/rbac')({
  component: RBACPage,
})

function RBACPage() {
  const { activeTab, setActiveTab } = useRBACStore()

  return (
    <div className="flex gap-2 flex-col h-full">
      <RBACHeader />

      <main className="flex-1 overflow-hidden">
        <Tabs
          value={activeTab}
          onValueChange={(val: RBACActiveTab) => setActiveTab(val)}
          className="h-full flex flex-col"
        >
          <TabsList className="w-full sm:w-fit">
            <TabsTrigger value="users">
              <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
              Users
            </TabsTrigger>
            <TabsTrigger value="roles">
              <HugeiconsIcon icon={Shield01Icon} className="size-4" />
              Roles
            </TabsTrigger>
            <TabsTrigger value="permission-sets">
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              Permission Sets
            </TabsTrigger>
          </TabsList>

          <TabsContent value="users">
            <UsersTab />
          </TabsContent>

          <TabsContent value="roles">
            <RolesTab />
          </TabsContent>

          <TabsContent value="permission-sets">
            <PermissionSetsTab />
          </TabsContent>
        </Tabs>
      </main>

      <RoleEditorDialog />
    </div>
  )
}
