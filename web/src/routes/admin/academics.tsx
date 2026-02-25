import { Outlet, createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/admin/academics')({
  component: AcademicsLayout,
})

function AcademicsLayout() {
  return <Outlet />
}
