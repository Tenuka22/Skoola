import { Link } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import { Menu01Icon } from '@hugeicons/core-free-icons'
import { useSidebar } from './sidebar-context'
import { Sheet, SheetContent, SheetTrigger } from '@/components/ui/sheet'
import { Button } from '@/components/ui/button'

export const AdminSidebar = () => {
  const { isOpen } = useSidebar()

  // For mobile, the sidebar will be a Sheet
  // For desktop, it will be a persistent element

  return (
    <>
      {/* Mobile Sidebar Trigger */}
      <div className="md:hidden fixed top-4 left-4 z-50">
        <Sheet>
          <SheetTrigger>
            <Button variant="outline" size="icon">
              <HugeiconsIcon icon={Menu01Icon} className="h-6 w-6" />
            </Button>
          </SheetTrigger>
          <SheetContent side="left" className="w-64 p-4">
            {/* Mobile Sidebar Content */}
            <h2 className="text-2xl font-bold mb-6">Skoola Admin</h2>
            <nav className="grid gap-2">
              <Link
                to={"/_admin/dashboard" as any}
                className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
              >
                Dashboard
              </Link>
              <Link
                to={"/_admin/staff" as any}
                className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
              >
                Staff Management
              </Link>
              <Link
                to={"/_admin/students" as any}
                className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
              >
                Student Management
              </Link>
              {/* More links as needed */}
            </nav>
          </SheetContent>
        </Sheet>
      </div>

      {/* Desktop Sidebar */}
      <aside
        className={`hidden md:flex flex-col h-screen w-64 p-4 border-r bg-background transition-all duration-300 ${isOpen ? 'w-64' : 'w-16'}`}
      >
        <h2 className={`text-2xl font-bold mb-6 ${!isOpen && 'sr-only'}`}>
          Skoola Admin
        </h2>
        <nav className="grid gap-2">
          <Link
            to={"/_admin/dashboard" as any}
            className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
          >
            Dashboard
          </Link>
          <Link
            to={"/_admin/staff" as any}
            className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
          >
            Staff Management
          </Link>
          <Link
            to={"/_admin/students" as any}
            className="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium hover:bg-muted transition-colors"
          >
            Student Management
          </Link>
          {/* More links as needed */}
        </nav>
      </aside>
    </>
  )
}
