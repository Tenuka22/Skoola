import type { PermissionEnum } from '@/lib/api/types.gen'

export const PERMISSION_CATEGORIES = [
  {
    name: 'User Management',
    permissions: [
      'UserCreate',
      'UserRead',
      'UserUpdate',
      'UserDelete',
      'UserManage',
      'UserManageRoles',
      'UserManagePermissions',
      'UserUpdateMedium',
      'UserDeleteSevere',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Role Management',
    permissions: [
      'RoleCreate',
      'RoleRead',
      'RoleUpdate',
      'RoleDelete',
      'RoleManage',
      'RoleAssignPermissions',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Permission Management',
    permissions: [
      'PermissionCreate',
      'PermissionRead',
      'PermissionUpdate',
      'PermissionDelete',
      'PermissionManage',
      'PermissionSetManage',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Staff Management',
    permissions: [
      'StaffCreate',
      'StaffRead',
      'StaffUpdate',
      'StaffDelete',
      'StaffManage',
      'StaffManageAttendance',
      'StaffManageLeaves',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Student Management',
    permissions: [
      'StudentCreate',
      'StudentRead',
      'StudentUpdate',
      'StudentDelete',
      'StudentManage',
      'StudentManageGuardians',
      'StudentManageEnrollment',
      'StudentManageAttendance',
      'StudentManageMarks',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Academic Management',
    permissions: [
      'AcademicYearManage',
      'TermManage',
      'GradeLevelManage',
      'ClassManage',
      'SubjectManage',
      'ClassSubjectTeacherManage',
      'TimetableManage',
      'ExamTypeManage',
      'ExamManage',
      'ExamSubjectManage',
      'GradingSchemeManage',
      'GradingCriterionManage',
    ] as Array<PermissionEnum>,
  },
  {
    name: 'Resource Management',
    permissions: ['LibraryManage'] as Array<PermissionEnum>,
  },
]
