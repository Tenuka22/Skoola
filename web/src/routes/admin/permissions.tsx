'use client'

import { createFileRoute } from '@tanstack/react-router'
import { RoleList } from '../../features/permissions/components/role-list'

export const Route = createFileRoute('/admin/permissions')({
  component: PermissionsPage,
})

function PermissionsPage() {
  return (
    <div className="space-y-10 p-6 max-w-7xl mx-auto">
      <div className="space-y-2">
        <h1 className="text-4xl font-black tracking-tight uppercase">Access Control Matrix</h1>
        <p className="text-muted-foreground font-medium">
          Define and enforce security boundaries across the Skoola infrastructure.
        </p>
      </div>

      <RoleList />
    </div>
  )
}
