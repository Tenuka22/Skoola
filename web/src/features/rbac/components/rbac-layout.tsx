import { HugeiconsIcon } from '@hugeicons/react'
import {
  Layers01Icon,
  Shield01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { isRBACActiveTab } from '../utils/permissions'
import { UsersTab } from './users-tab'
import { RolesTab } from './roles-tab'
import { PermissionSetsTab } from './permission-sets-tab'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

export function RBACLayout() {
  const { activeTab, setActiveTab } = useRBACStore()

  return (
    <div className="flex flex-col h-full gap-6 p-8">
      <div className="flex flex-col gap-1">
        <h1 className="text-3xl font-bold tracking-tight">Access Control</h1>
        <p className="text-muted-foreground">
          Manage user permissions, system roles, and custom permission sets.
        </p>
      </div>

      <Tabs
        defaultValue="users"
        value={activeTab}
        onValueChange={(val: string) => {
          if (isRBACActiveTab(val)) {
            setActiveTab(val)
          }
        }}
        className="flex-1 flex flex-col gap-6 overflow-hidden"
      >
        <TabsList className="w-fit p-1 bg-muted/50 border">
          <TabsTrigger value="users" className="gap-2 px-4 py-2">
            <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
            Users
          </TabsTrigger>
          <TabsTrigger value="roles" className="gap-2 px-4 py-2">
            <HugeiconsIcon icon={Shield01Icon} className="size-4" />
            Roles
          </TabsTrigger>
          <TabsTrigger value="permission-sets" className="gap-2 px-4 py-2">
            <HugeiconsIcon icon={Layers01Icon} className="size-4" />
            Permission Sets
          </TabsTrigger>
        </TabsList>

        <div className="flex-1 overflow-hidden">
          <TabsContent
            value="users"
            className="h-full m-0 data-[state=inactive]:hidden"
          >
            <UsersTab />
          </TabsContent>

          <TabsContent
            value="roles"
            className="h-full m-0 data-[state=inactive]:hidden overflow-y-auto pr-2"
          >
            <RolesTab />
          </TabsContent>

          <TabsContent
            value="permission-sets"
            className="h-full m-0 data-[state=inactive]:hidden"
          >
            <PermissionSetsTab />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  )
}
