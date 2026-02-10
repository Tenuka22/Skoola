import { PermissionEnum, PermissionSeverity } from '@/lib/api/types.gen'

export const PERMISSION_NAMES: PermissionEnum[] = [
  'UserCreate', 'UserRead', 'UserUpdate', 'UserDelete', 'UserManage',
  'UserManageRoles', 'UserManagePermissions', 'RoleCreate', 'RoleRead',
  'RoleUpdate', 'RoleDelete', 'RoleManage', 'RoleAssignPermissions',
  'PermissionCreate', 'PermissionRead', 'PermissionUpdate', 'PermissionDelete',
  'PermissionManage', 'PermissionSetManage', 'StaffCreate', 'StaffRead',
  'StaffUpdate', 'StaffDelete', 'StaffManage', 'StaffManageAttendance',
  'StaffManageLeaves', 'StudentCreate', 'StudentRead', 'StudentUpdate',
  'StudentDelete', 'StudentManage', 'StudentManageGuardians',
  'StudentManageEnrollment', 'StudentManageAttendance', 'StudentManageMarks',
  'AcademicYearManage', 'TermManage', 'GradeLevelManage', 'ClassManage',
  'SubjectManage', 'ClassSubjectTeacherManage', 'TimetableManage',
  'ExamTypeManage', 'ExamManage', 'ExamSubjectManage', 'GradingSchemeManage',
  'GradingCriterionManage', 'LibraryManage', 'UserUpdateMedium', 'UserDeleteSevere'
]

export const PERMISSION_SEVERITIES: PermissionSeverity[] = ['Low', 'Medium', 'High', 'Severe']