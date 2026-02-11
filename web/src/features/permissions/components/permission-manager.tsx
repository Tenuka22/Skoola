'use client'

import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  DashboardSquare01Icon,
  Delete02Icon,
  LibraryIcon,
//   Loading03Icon,
  Mortarboard01Icon,
  SecurityIcon,
  Shield01Icon,
  Task01Icon,
  UserGroupIcon,
  ZapIcon,
} from '@hugeicons/core-free-icons'
// import { useMutation, useQueryClient } from '@tanstack/react-query'
// import { toast } from 'sonner'
// import { createPermission, deletePermission } from '../api'
// import type { Permission, PermissionEnum, PermissionSeverity  } from '@/lib/api/types.gen'
import { Switch } from '@/components/ui/switch'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'


interface PermissionManagerProps {
  permissions: Array<any>
  assignedPermissionIds: Array<number>
  onToggle: (permissionId: number, isEnabled: boolean) => void
  readOnly?: boolean
}

export function PermissionManager({
  permissions,
  assignedPermissionIds,
  onToggle,
  readOnly = false,
}: PermissionManagerProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [newName, setNewName] = React.useState<string>('UserRead')
  const [newDesc, setNewDesc] = React.useState('')
  const [newSeverity, setNewSeverity] = React.useState<string>('Low')
  
//   const queryClient = useQueryClient()

//   const createMutation = useMutation({
//     mutationFn: () => createPermission(newName, newDesc, newSeverity),
//     onSuccess: () => {
//       queryClient.invalidateQueries({ queryKey: ['permissions'] })
//       setIsCreateOpen(false)
//       setNewDesc('')
//       toast.success('Permission added to global registry')
//     },
//     onError: () => toast.error('Failed to register permission'),
//   })

//   const deleteMutation = useMutation({
//     mutationFn: (id: number) => deletePermission(id),
//     onSuccess: () => {
//       queryClient.invalidateQueries({ queryKey: ['permissions'] })
//       toast.success('Permission removed from registry')
//     },
//     onError: () => toast.error('Failed to purge permission'),
//   })

// Group permissions by prefix/category
  const categories: Record<string, Array<any>> = {
    'User Management': permissions.filter((p) => p.name.startsWith('User')),
    'Role Management': permissions.filter((p) => p.name.startsWith('Role')),
    'Permission Management': permissions.filter((p) =>
      p.name.startsWith('Permission'),
    ),
    'Staff Management': permissions.filter((p) => p.name.startsWith('Staff')),
    'Student Management': permissions.filter((p) =>
      p.name.startsWith('Student'),
    ),
    'Academic Infrastructure': permissions.filter(
      (p) =>
        p.name.includes('Academic') ||
        p.name.includes('Term') ||
        p.name.includes('GradeLevel') ||
        p.name.includes('Class') ||
        p.name.includes('Subject') ||
        p.name.includes('Timetable'),
    ),
    'Exams & Grading': permissions.filter(
      (p) => p.name.includes('Exam') || p.name.includes('Grading'),
    ),
    'Library & Other': permissions.filter((p) => p.name.includes('Library')),
  }

  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'User Management':
        return UserGroupIcon
      case 'Role Management':
        return Shield01Icon
      case 'Permission Management':
        return SecurityIcon
      case 'Staff Management':
        return Mortarboard01Icon
      case 'Student Management':
        return Mortarboard01Icon
      case 'Academic Infrastructure':
        return DashboardSquare01Icon
      case 'Exams & Grading':
        return Task01Icon
      case 'Library & Other':
        return LibraryIcon
      default:
        return ZapIcon
    }
  }

  const getSeverityStyles = (severity: string) => {
    switch (severity) {
      case 'Low':
        return 'text-green-500 bg-green-500/10 border-green-500/20'
      case 'Medium':
        return 'text-blue-500 bg-blue-500/10 border-blue-500/20'
      case 'High':
        return 'text-orange-500 bg-orange-500/10 border-orange-500/20'
      case 'Severe':
        return 'text-red-500 bg-red-500/10 border-red-500/20'
      default:
        return 'text-muted-foreground bg-muted border-transparent'
    }
  }

  const availableEnumVariants: Array<string> = [
      'UserCreate', 'UserRead', 'UserUpdate', 'UserDelete', 'UserManage', 'UserManageRoles', 'UserManagePermissions',
      'RoleCreate', 'RoleRead', 'RoleUpdate', 'RoleDelete', 'RoleManage', 'RoleAssignPermissions',
      'PermissionCreate', 'PermissionRead', 'PermissionUpdate', 'PermissionDelete', 'PermissionManage',
      'StaffCreate', 'StaffRead', 'StaffUpdate', 'StaffDelete', 'StaffManage', 'StaffManageAttendance', 'StaffManageLeaves',
      'StudentCreate', 'StudentRead', 'StudentUpdate', 'StudentDelete', 'StudentManage', 'StudentManageGuardians', 'StudentManageEnrollment', 'StudentManageAttendance', 'StudentManageMarks',
      'AcademicYearManage', 'TermManage', 'GradeLevelManage', 'ClassManage', 'SubjectManage', 'ClassSubjectTeacherManage', 'TimetableManage',
      'ExamTypeManage', 'ExamManage', 'ExamSubjectManage', 'GradingSchemeManage', 'GradingCriterionManage',
      'LibraryManage', 'UserUpdateMedium', 'UserDeleteSevere'
  ]

  return (
    <div className="space-y-12">
      <div className="flex justify-end">
        <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
          {!readOnly && (
            <DialogTrigger
              render={
                <Button
                  variant="outline"
                  className="h-10 rounded-xl font-black uppercase text-[10px] tracking-widest gap-2 bg-background/50 ring-1 ring-border shadow-sm transition-all hover:bg-primary hover:text-primary-foreground hover:ring-primary"
                >
                  <HugeiconsIcon icon={Add01Icon} className="size-4" />
                  Register Capability
                </Button>
              }
            />
          )}
          <DialogContent className="rounded-[2rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
            <DialogHeader>
              <div className="mx-auto mb-4 flex size-16 items-center justify-center rounded-2xl bg-primary/10 text-primary">
                <HugeiconsIcon icon={SecurityIcon} className="size-8" />
              </div>
              <DialogTitle className="text-center text-2xl font-black uppercase tracking-tight">Register Capability</DialogTitle>
              <DialogDescription className="text-center">
                Add a static capability from the system manifest to the live mesh.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-6 py-6">
              <div className="space-y-2">
                <Label className="text-[10px] font-black uppercase tracking-widest opacity-50">Capability Manifest Name</Label>
                <Select value={newName} onValueChange={(val) => setNewName(val ?? '')}>
                  <SelectTrigger className="h-12 rounded-xl bg-muted/30 border-none px-4 font-bold capitalize">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {availableEnumVariants.map(v => (
                        <SelectItem key={v} value={v}>{v}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2">
                <Label className="text-[10px] font-black uppercase tracking-widest opacity-50">Operational Description</Label>
                <Input
                  value={newDesc}
                  onChange={(e) => setNewDesc(e.target.value)}
                  placeholder="What does this permission enable?"
                  className="h-12 rounded-xl bg-muted/30 border-none px-4 font-bold"
                />
              </div>
              <div className="space-y-2">
                <Label className="text-[10px] font-black uppercase tracking-widest opacity-50">Safety Level</Label>
                <Select value={newSeverity} onValueChange={(val) => setNewSeverity(val ?? '')}>
                  <SelectTrigger className="h-12 rounded-xl bg-muted/30 border-none px-4 font-bold">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {['Low', 'Medium', 'High', 'Severe'].map(v => (
                        <SelectItem key={v} value={v}>{v}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            <DialogFooter>
              <Button variant="ghost" onClick={() => setIsCreateOpen(false)} className="h-12 rounded-xl font-black uppercase tracking-widest text-[10px]">Abort</Button>
              <Button
                disabled={!newDesc}
                // onClick={() => createMutation.mutate()}
                className="h-12 px-8 rounded-xl font-black uppercase tracking-widest text-[10px] shadow-xl shadow-primary/20"
              >
                Register Mesh Unit
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-10">
        {Object.entries(categories).map(([category, perms]) => {
          if (perms.length === 0) return null
          return (
            <div key={category} className="space-y-6">
              <div className="flex items-center gap-3 mb-2">
                <div className="p-2.5 rounded-xl bg-primary/10 text-primary ring-1 ring-primary/20 shadow-inner">
                  <HugeiconsIcon icon={getCategoryIcon(category)} className="size-5" />
                </div>
                <h3 className="text-[11px] font-black uppercase tracking-[0.2em] opacity-60">
                  {category}
                </h3>
              </div>

              <div className="space-y-3">
                {perms.map((permission) => {
                  const isAssigned = assignedPermissionIds.includes(permission.id)
                  return (
                    <div
                      key={permission.id}
                      className="group relative flex items-center justify-between p-5 rounded-3xl bg-muted/20 border border-transparent transition-all hover:bg-muted/40 hover:ring-1 hover:ring-primary/20"
                    >
                      <div className="space-y-2 flex-1 mr-4">
                        <div className="flex items-center gap-2">
                          <span className="text-sm font-black tracking-tight">
                            {permission.name}
                          </span>
                          <Badge
                            variant="outline"
                            className={cn(
                              'h-4 px-1.5 text-[7px] font-black uppercase border tracking-tighter',
                              getSeverityStyles(permission.safety_level),
                            )}
                          >
                            {permission.safety_level}
                          </Badge>
                          {permission.is_admin_only && (
                            <Badge
                              className="h-4 px-1.5 text-[7px] font-black uppercase bg-primary text-primary-foreground tracking-tighter"
                            >
                              Admin
                            </Badge>
                          )}
                        </div>
                        <p className="text-[10px] font-medium text-muted-foreground leading-relaxed">
                          {permission.description}
                        </p>
                      </div>
                      
                      <div className="flex items-center gap-4">
                        {!readOnly && (
                            <Button 
                                variant="ghost" 
                                size="icon" 
                                className="size-8 rounded-lg text-destructive opacity-0 group-hover:opacity-100 transition-all hover:bg-destructive/10"
                                // onClick={() => deleteMutation.mutate(permission.id)}
                            >
                                <HugeiconsIcon icon={Delete02Icon} className="size-3" />
                            </Button>
                        )}
                        <Switch
                          checked={isAssigned}
                          disabled={readOnly}
                          onCheckedChange={(checked) =>
                            onToggle(permission.id, checked)
                          }
                          className="data-[state=checked]:bg-primary"
                        />
                      </div>
                    </div>
                  )
                })}
              </div>
            </div>
          )
        })}
      </div>
    </div>
  )
}
