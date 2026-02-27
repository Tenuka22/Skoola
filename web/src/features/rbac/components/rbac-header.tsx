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
    <div className="px-8 py-6 pb-2">
      <div className="mb-1 flex items-center gap-3">
        <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {count}
        </Badge>
      </div>
      <p className="text-sm text-muted-foreground">{description}</p>
    </div>
  )
}
