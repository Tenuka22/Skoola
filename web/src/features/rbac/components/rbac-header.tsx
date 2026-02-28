import { useRBACStore } from '../store'
import { Badge } from '@/components/ui/badge'
import { HStack, Heading, Stack, Text } from '@/components/primitives'

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
      case 'role-sets':
        return {
          title: 'Role Sets',
          description:
            'Define groups of roles that can be managed and assigned together.',
          count: 'Role groupings',
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
    <Stack gap={1}>
      <HStack align="center" gap={3}>
        <Heading size="h2">{title}</Heading>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {count}
        </Badge>
      </HStack>
      <Text muted as="p">
        {description}
      </Text>
    </Stack>
  )
}
