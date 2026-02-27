import { HugeiconsIcon } from '@hugeicons/react'
import {
  AccessIcon,
  Shield01Icon,
  UserGroupIcon,
  Layers01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { Badge } from '@/components/ui/badge'

export function RBACHeader() {
  const { activeTab } = useRBACStore()

  const getHeaderInfo = () => {
    switch (activeTab) {
      case 'users':
        return {
          title: 'User Permissions',
          description:
            'Manage direct permission assignments and roles for individual users.',
          count: 'Direct access control',
        }
      case 'roles':
        return {
          title: 'System Roles',
          description:
            'Configure baseline permissions for system-wide roles (Admin, Teacher, etc).',
          count: 'Global role management',
        }
      case 'permission-sets':
        return {
          title: 'Permission Sets',
          description:
            'Create reusable bundles of permissions that can be assigned to staff.',
          count: 'Customizable sets',
        }
      default:
        return {
          title: 'Access Control',
          description:
            'Manage user permissions, system roles, and custom permission sets.',
          count: 'RBAC Overview',
        }
    }
  }

  const { title, description, count } = getHeaderInfo()

  return (
    <div className="flex items-center justify-between gap-4 px-8 py-6 pb-2">
      <div className="flex flex-col">
        <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
      <Badge variant="secondary">{count}</Badge>
    </div>
  )
}
