'use client'

import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  RefreshIcon,
  Shield01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { PermissionsTable } from '../../features/permissions/components/permissions-table'
import { PermissionSetsList } from '../../features/permissions/components/permission-sets-list'

import { isPermissionSetArray } from '../../features/permissions/utils/permission-guards'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import { authClient } from '@/lib/clients'
import {
  getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44Options as getPermissionSetsOptions,
  getPermissions9C8839E73223Cb930255A2882A4B0Db4Options as getPermissionsOptions,
} from '@/lib/api/@tanstack/react-query.gen'

export const Route = createFileRoute('/admin/permissions')({
  component: PermissionsPage,
})

function PermissionsPage() {
  // State for search, pagination, sorting for permissions
  const [permissionsPage, setPermissionsPage] = React.useState(1)
  const [permissionsSearch, setPermissionsSearch] = React.useState('')
  const [permissionsSortBy, setPermissionsSortBy] = React.useState('name')
  const [permissionsSortOrder, setPermissionsSortOrder] = React.useState('asc')
  const permissionsLimit = 10

  // Fetch permissions
  const {
    data: permissionsData,
    isLoading: isLoadingPermissions,
    refetch: refetchPermissions,
  } = useQuery({
    ...getPermissionsOptions({
      client: authClient,
      query: {
        page: permissionsPage,
        limit: permissionsLimit,
        search: permissionsSearch,
        sort_by: permissionsSortBy,
        sort_order: permissionsSortOrder,
      },
    }),
    placeholderData: (previousData) => previousData, // Keep previous data for smooth transitions
  })

  // Fetch permission sets
  const {
    data: permissionSetsData,
    isLoading: isLoadingPermissionSets,
    refetch: refetchPermissionSets,
  } = useQuery({
    ...getPermissionSetsOptions({
      client: authClient,
    }),
    placeholderData: (previousData) => previousData,
    // Assuming getPermissionSets returns an array of PermissionSet directly, adjust if it's paginated.
    select: (data) => (isPermissionSetArray(data) ? data : []),
  })

  return (
    <div className="space-y-10 p-6 max-w-7xl mx-auto">
      <div className="space-y-2">
        <h1 className="text-4xl font-black tracking-tight uppercase">
          Access Control Matrix
        </h1>
        <p className="text-muted-foreground font-medium">
          Define and enforce security boundaries across the Skoola
          infrastructure.
        </p>
      </div>

      <Tabs defaultValue="permissions" className="space-y-6">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <TabsList className="grid w-full grid-cols-2 lg:w-[400px]">
            <TabsTrigger value="permissions">
              <HugeiconsIcon icon={Shield01Icon} className="mr-2 size-4" />
              Permissions
            </TabsTrigger>
            <TabsTrigger value="permission-sets">
              <HugeiconsIcon icon={UserGroupIcon} className="mr-2 size-4" />
              Permission Sets
            </TabsTrigger>
          </TabsList>
          <Button
            variant="outline"
            size="icon"
            className="h-10 w-10 rounded-xl shadow-sm transition-transform active:scale-95"
            onClick={() => {
              refetchPermissions()
              refetchPermissionSets()
            }}
          >
            <HugeiconsIcon icon={RefreshIcon} className="size-4" />
          </Button>
        </div>

        <TabsContent value="permissions" className="space-y-4">
          <Card className="border-none shadow-xl overflow-hidden">
            <CardHeader className="flex flex-col space-y-4 border-b bg-muted/20 px-6 py-5 lg:flex-row lg:items-center lg:justify-between lg:space-y-0">
              <div className="space-y-1">
                <CardTitle className="text-2xl font-black">
                  Global Permissions
                </CardTitle>
                <CardDescription className="font-medium">
                  Manage individual permissions within the system.
                </CardDescription>
              </div>
              {/* Search and filter controls for permissions will go here */}
            </CardHeader>
            <CardContent className="p-0">
              <PermissionsTable
                permissions={permissionsData?.data || []}
                isLoading={isLoadingPermissions}
                page={permissionsPage}
                limit={permissionsLimit}
                totalPages={permissionsData?.total_pages || 0}
                setPage={setPermissionsPage}
                setSearch={setPermissionsSearch}
                setSorting={(updater) => {
                  const sortingState =
                    typeof updater === 'function' ? updater([]) : updater
                  if (sortingState.length > 0) {
                    setPermissionsSortBy(sortingState[0].id)
                    setPermissionsSortOrder(
                      sortingState[0].desc ? 'desc' : 'asc',
                    )
                  } else {
                    setPermissionsSortBy('name')
                    setPermissionsSortOrder('asc')
                  }
                }}
              />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="permission-sets" className="space-y-4">
          <Card className="border-none shadow-xl overflow-hidden">
            <CardHeader className="flex flex-col space-y-4 border-b bg-muted/20 px-6 py-5 lg:flex-row lg:items-center lg:justify-between lg:space-y-0">
              <div className="space-y-1">
                <CardTitle className="text-2xl font-black">
                  Permission Sets
                </CardTitle>
                <CardDescription className="font-medium">
                  Group permissions into reusable sets.
                </CardDescription>
              </div>
              {/* Search and filter controls for permission sets will go here */}
            </CardHeader>
            <CardContent className="p-0">
              <PermissionSetsList
                permissionSets={permissionSetsData || []}
                isLoading={isLoadingPermissionSets}
                allPermissions={permissionsData?.data || []} // Pass all permissions for managing set permissions
              />
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
