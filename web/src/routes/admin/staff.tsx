'use client'

import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useMutation, useQuery, useQueryClient, keepPreviousData } from '@tanstack/react-query'
import { 
  getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options,
  deleteStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation,
  postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation,
  putStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation
} from '@/lib/api/@tanstack/react-query.gen'
import { StaffCard } from '@/features/staff/components/staff-card'
import { StaffModals } from '@/features/staff/components/staff-modals'
import { Button } from '@/components/ui/button'
import { HugeiconsIcon } from '@hugeicons/react'
import { 
  PlusSignIcon, 
  Upload02Icon, 
  Download02Icon,
  Search01Icon,
  FilterIcon,
  ArrowLeft01Icon,
  ArrowRight01Icon,
  RefreshIcon
} from '@hugeicons/core-free-icons'
import { Input } from '@/components/ui/input'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
} from '@/components/ui/dropdown-menu'
import { authClient } from '@/lib/clients'
import { toast } from 'sonner'
import type { StaffType, EmploymentStatus, StaffResponse } from '@/lib/api/types.gen'

export const Route = createFileRoute('/admin/staff')({
  component: StaffPage,
})

function StaffPage() {
  const [page, setPage] = React.useState(1)
  const [search, setSearch] = React.useState('')
  const [debouncedSearch, setDebouncedSearch] = React.useState('')
  const [staffTypeFilter, setStaffTypeFilter] = React.useState<string>('all')
  const [employmentStatusFilter, setEmploymentStatusFilter] = React.useState<string>('all')
  
  const [staffToDelete, setStaffToDelete] = React.useState<StaffResponse | null>(null)
  const [staffToEdit, setStaffToEdit] = React.useState<StaffResponse | null>(null)
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const limit = 9 // 3x3 grid
  const queryClient = useQueryClient()

  // Debounce search
  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
      setPage(1)
    }, 400)
    return () => clearTimeout(handler)
  }, [search])

  const { data, isLoading, error, refetch } = useQuery({
    ...getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        staff_type: staffTypeFilter === 'all' ? undefined : staffTypeFilter as StaffType,
        employment_status: employmentStatusFilter === 'all' ? undefined : employmentStatusFilter as EmploymentStatus,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteMutation = useMutation({
    ...deleteStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Staff member removed')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857' }],
      })
      setStaffToDelete(null)
    },
    onError: (error: any) => {
      toast.error(`Failed to delete staff: ${error.message}`)
    }
  })

  const addMutation = useMutation({
    ...postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('New employee registered')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857' }],
      })
      setIsAddOpen(false)
    },
    onError: (error: any) => {
      toast.error(`Failed to add staff: ${error.message}`)
    }
  })

  const updateMutation = useMutation({
    ...putStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Records updated')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857' }],
      })
      setStaffToEdit(null)
    },
    onError: (error: any) => {
      toast.error(`Failed to update staff: ${error.message}`)
    }
  })

  const staffMembers = data?.data || []
  const totalEmployees = data?.total || 0
  const totalPages = data?.total_pages || 0

  if (error) {
    return (
      <div className="p-8 text-center">
        <p className="text-red-500 mb-4">Error loading staff: {error.message}</p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  return (
    <div className="p-8 space-y-8">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h1 className="text-3xl font-black tracking-tight">{totalEmployees} Staff Members</h1>
          <p className="text-muted-foreground font-medium">Manage your educational institution's human capital.</p>
        </div>
        <div className="flex flex-wrap gap-2">
          <Button variant="outline" className="rounded-xl h-11">
            <HugeiconsIcon icon={Upload02Icon} className="w-4 h-4 mr-2" /> Import
          </Button>
          <Button variant="outline" className="rounded-xl h-11">
            <HugeiconsIcon icon={Download02Icon} className="w-4 h-4 mr-2" /> Export
          </Button>
          <Button 
            className="rounded-xl h-11 bg-primary text-primary-foreground shadow-lg shadow-primary/20"
            onClick={() => setIsAddOpen(true)}
          >
            <HugeiconsIcon icon={PlusSignIcon} className="w-4 h-4 mr-2" /> Add Member
          </Button>
        </div>
      </div>

      <div className="flex flex-col lg:flex-row justify-between items-stretch lg:items-center gap-4">
        <div className="relative flex-1 max-w-md group">
          <HugeiconsIcon 
            icon={Search01Icon} 
            className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground group-focus-within:text-primary transition-colors" 
          />
          <Input 
            placeholder="Search by name, email or ID..." 
            className="pl-10 h-11 rounded-xl border-none bg-muted/50 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        
        <div className="flex flex-wrap items-center gap-2">
          <DropdownMenu>
            <DropdownMenuTrigger render={
              <Button variant="outline" className="h-11 rounded-xl border-none bg-muted/50 ring-1 ring-border">
                <HugeiconsIcon icon={FilterIcon} className="w-4 h-4 mr-2" />
                {staffTypeFilter === 'all' ? 'All Roles' : staffTypeFilter}
              </Button>
            } />
            <DropdownMenuContent align="end" className="w-56 rounded-xl p-2">
              <DropdownMenuLabel>Staff Type</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuRadioGroup value={staffTypeFilter} onValueChange={setStaffTypeFilter}>
                <DropdownMenuRadioItem value="all">All Roles</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="Teaching">Teaching</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="NonTeaching">Non-Teaching</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="Administrative">Administrative</DropdownMenuRadioItem>
              </DropdownMenuRadioGroup>
            </DropdownMenuContent>
          </DropdownMenu>

          <DropdownMenu>
            <DropdownMenuTrigger render={
              <Button variant="outline" className="h-11 rounded-xl border-none bg-muted/50 ring-1 ring-border">
                <HugeiconsIcon icon={FilterIcon} className="w-4 h-4 mr-2" />
                {employmentStatusFilter === 'all' ? 'All Status' : employmentStatusFilter}
              </Button>
            } />
            <DropdownMenuContent align="end" className="w-56 rounded-xl p-2">
              <DropdownMenuLabel>Employment Status</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuRadioGroup value={employmentStatusFilter} onValueChange={setEmploymentStatusFilter}>
                <DropdownMenuRadioItem value="all">All Status</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="Permanent">Permanent</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="Contract">Contract</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="Temporary">Temporary</DropdownMenuRadioItem>
              </DropdownMenuRadioGroup>
            </DropdownMenuContent>
          </DropdownMenu>

          <Button 
            variant="outline" 
            size="icon" 
            className="h-11 w-11 rounded-xl border-none bg-muted/50 ring-1 ring-border shadow-sm active:scale-95 transition-transform"
            onClick={() => refetch()}
          >
            <HugeiconsIcon icon={RefreshIcon} className="w-4 h-4" />
          </Button>
        </div>
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="h-64 rounded-3xl bg-muted/50 animate-pulse" />
          ))}
        </div>
      ) : staffMembers.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {staffMembers.map((staff) => (
            <StaffCard 
              key={staff.id} 
              staff={staff} 
              onEdit={(s) => setStaffToEdit(s)}
              onDelete={(s) => setStaffToDelete(s)}
            />
          ))}
        </div>
      ) : (
        <div className="py-20 text-center bg-muted/20 rounded-[2.5rem] border-2 border-dashed border-border">
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-muted text-muted-foreground">
            <HugeiconsIcon icon={Search01Icon} className="size-10" />
          </div>
          <h3 className="text-xl font-bold">No staff members found</h3>
          <p className="text-muted-foreground">Try adjusting your search or filters.</p>
          <Button variant="link" onClick={() => {
            setSearch('')
            setStaffTypeFilter('all')
            setEmploymentStatusFilter('all')
          }}>Clear all filters</Button>
        </div>
      )}

      <div className="flex flex-col sm:flex-row justify-between items-center gap-4 pt-4">
        <p className="text-sm font-medium text-muted-foreground">
          Showing <span className="text-foreground">{staffMembers.length > 0 ? (page - 1) * limit + 1 : 0}</span> to{' '}
          <span className="text-foreground">{Math.min(page * limit, totalEmployees)}</span> of{' '}
          <span className="text-foreground">{totalEmployees}</span> entries
        </p>
        <div className="flex items-center gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            className="rounded-xl h-10 px-4 font-bold border-none bg-muted/50 ring-1 ring-border disabled:opacity-30"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page === 1}
          >
            <HugeiconsIcon icon={ArrowLeft01Icon} className="w-4 h-4 mr-2" /> Previous
          </Button>
          <div className="flex items-center gap-1">
            {totalPages > 0 && [...Array(totalPages)].map((_, i) => {
              const p = i + 1
              if (totalPages > 5 && Math.abs(p - page) > 2 && p !== 1 && p !== totalPages) {
                if (p === 2 || p === totalPages - 1) return <span key={p}>...</span>
                return null
              }
              return (
                <Button
                  key={p}
                  variant={page === p ? 'default' : 'outline'}
                  size="sm"
                  className={`w-10 h-10 rounded-xl font-bold transition-all ${page === p ? 'shadow-lg shadow-primary/20' : 'border-none bg-muted/50 ring-1 ring-border'}`}
                  onClick={() => setPage(p)}
                >
                  {p}
                </Button>
              )
            })}
          </div>
          <Button 
            variant="outline" 
            size="sm" 
            className="rounded-xl h-10 px-4 font-bold border-none bg-muted/50 ring-1 ring-border disabled:opacity-30"
            onClick={() => setPage((p) => p + 1)}
            disabled={page >= totalPages}
          >
            Next <HugeiconsIcon icon={ArrowRight01Icon} className="w-4 h-4 ml-2" />
          </Button>
        </div>
      </div>

      <StaffModals
        staffToDelete={staffToDelete}
        setStaffToDelete={setStaffToDelete}
        onDeleteConfirm={(id) => deleteMutation.mutate({ path: { staff_id: id } })}
        isAddOpen={isAddOpen}
        setIsAddOpen={setIsAddOpen}
        onAddConfirm={(values) => addMutation.mutate({ body: values })}
        isAdding={addMutation.isPending}
        staffToEdit={staffToEdit}
        setStaffToEdit={setStaffToEdit}
        onEditConfirm={(values) => staffToEdit && updateMutation.mutate({ path: { staff_id: staffToEdit.id }, body: values })}
        isEditing={updateMutation.isPending}
      />
    </div>
  )
}
