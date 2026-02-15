import { createFileRoute } from '@tanstack/react-router'
import { RBACLayout } from '@/features/rbac/components/rbac-layout'

export const Route = createFileRoute('/admin/rbac')({
  component: RBACPage,
})

function RBACPage() {
  return <RBACLayout />
}
