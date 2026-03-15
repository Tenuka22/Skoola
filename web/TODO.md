# RBAC & Users Management - Backend Improvement TODO

This document outlines missing backend functionality and recommended improvements to enhance the RBAC (Role-Based Access Control) and Users management system in Skoola.

---

## 🔴 Critical Missing Features

### 1. User Management API Enhancements

#### 1.1 User Export Functionality

**Current State:** No export functionality for user data.

**Recommended Endpoint:**

```
GET /admin/users/export
Query Parameters:
  - format: 'csv' | 'json'
  - ids?: string[] (optional, for bulk export of specific users)
  - filters?: {
      search?: string
      status?: 'verified' | 'unverified'
      auth_method?: 'password' | 'google' | 'github'
      role?: RoleEnum
      created_after?: string
      created_before?: string
    }

Response: File download (application/csv or application/json)
```

**Use Cases:**

- Backup user data
- External reporting
- Migration to other systems
- Compliance audits

#### 1.2 User Import Wizard Support

**Current State:** `bulkImportUsers` hook exists but no UI trigger.

**Recommended Enhancements:**

```
POST /admin/users/import/preview
Body: { file: FormData (CSV/JSON) }
Response: {
  valid_rows: Array<User>
  invalid_rows: Array<{ row: number, errors: string[] }>
  summary: { total: number, valid: number, invalid: number }
}

POST /admin/users/import/confirm
Body: { import_id: string, conflicts_strategy: 'skip' | 'update' | 'overwrite' }
Response: { imported: number, skipped: number, failed: number }
```

**Use Cases:**

- Preview data before import
- Handle conflicts gracefully
- Validate data integrity
- Batch import from external systems

#### 1.3 User Security Settings Management

**Current State:** No API for managing user security settings.

**Recommended Endpoints:**

```
POST /admin/users/{id}/security/reset-password
Body: { send_email?: boolean, temporary_password?: string }
Response: { success: boolean, temporary_password?: string }

POST /admin/users/{id}/security/2fa/disable
Body: { reason?: string }
Response: { success: boolean }

GET /admin/users/{id}/security/sessions
Response: { sessions: Array<{ id, device, ip, last_active, created_at }> }

DELETE /admin/users/{id}/security/sessions/{session_id}
Response: { success: boolean }

POST /admin/users/{id}/security/lock
Body: {
  lockout_until: string (ISO 8601),
  reason?: string
}
Response: { success: boolean }

POST /admin/users/{id}/security/unlock
Response: { success: boolean }
```

**Use Cases:**

- IT support for locked accounts
- Security incident response
- Session management
- Forced password resets

#### 1.4 User Activity & Audit Logs

**Current State:** No visibility into user activity.

**Recommended Endpoints:**

```
GET /admin/users/{id}/activity
Query Parameters:
  - page?: number
  - limit?: number
  - action_type?: 'login' | 'logout' | 'permission_change' | 'profile_update' | 'password_change'
  - from?: string (ISO 8601)
  - to?: string (ISO 8601)

Response: {
  data: Array<{
    id: string
    user_id: string
    action: string
    ip_address?: string
    user_agent?: string
    metadata?: Record<string, any>
    created_at: string
  }>
  total: number
  page: number
  limit: number
}
```

**Use Cases:**

- Security auditing
- Compliance requirements
- Troubleshooting user issues
- Usage analytics

---

## 🟡 RBAC Enhancements

### 2. Permission Set Management

#### 2.1 Permission Set Templates

**Current State:** Permission sets must be created manually.

**Recommended Enhancement:**

```
GET /admin/permission-sets/templates
Response: {
  templates: Array<{
    id: string
    name: string
    description: string
    category: 'academic' | 'administrative' | 'finance' | 'support'
    permissions: PermissionEnum[]
  }>
}

POST /admin/permission-sets/from-template
Body: {
  template_id: string,
  name: string,
  description?: string
}
Response: { id: string, name: string }
```

**Use Cases:**

- Quick setup for common roles (e.g., "Math Teacher", "Grade Advisor")
- Consistency across permission sets
- Best practices enforcement

#### 2.2 Permission Set Cloning

```
POST /admin/permission-sets/{id}/clone
Body: { name: string, description?: string }
Response: { id: string, name: string }
```

#### 2.3 Permission Set Usage Analysis

```
GET /admin/permission-sets/{id}/usage
Response: {
  assigned_to_users: number
  assigned_to_staff: number
  user_ids: string[]
  staff_ids: string[]
  last_assigned_at?: string
}
```

### 3. Role Set Enhancements

#### 3.1 Role Set Hierarchy

**Current State:** Role sets are flat collections of roles.

**Recommended Enhancement:**
Support nested role sets for complex organizational structures.

```
POST /admin/role-sets/{id}/inherit
Body: { inherits_from_id: string }
Response: { success: boolean }

GET /admin/role-sets/{id}/inherited-roles
Response: {
  direct_roles: RoleEnum[]
  inherited_roles: Array<{
    role: RoleEnum
    source_role_set_id: string
    source_role_set_name: string
  }>
}
```

**Use Cases:**

- Multi-level organizational structures
- Inherited permissions for departments
- Reduced duplication

#### 3.2 Role Set Assignment to Users

**Current State:** Users can only be assigned individual roles, not role sets.

**Recommended Endpoint:**

```
POST /admin/users/{user_id}/role-sets/{role_set_id}
Response: { success: boolean }

DELETE /admin/users/{user_id}/role-sets/{role_set_id}
Response: { success: boolean }

GET /admin/users/{user_id}/role-sets
Response: {
  role_sets: Array<RoleSet & { assigned_at: string }>
}
```

**Use Cases:**

- Simplified role management
- Bulk role assignment
- Organizational role grouping

### 4. Staff Permission Set Integration

#### 4.1 Staff Permission Set Management UI

**Current State:** API hooks exist (`useAssignPermissionSetToStaff`, `getStaffPermissionSetsQueryOptions`) but no UI.

**Recommended Implementation:**
Create a new tab in `/admin/rbac` for "Staff Permissions" or integrate into existing Staff feature.

```
GET /admin/staff/{staff_id}/permission-sets
Response: {
  permission_sets: Array<UserSet & { assigned_at: string }>
}

POST /admin/staff/{staff_id}/permission-sets
Body: { permission_set_id: string }
Response: { success: boolean }

DELETE /admin/staff/{staff_id}/permission-sets/{permission_set_id}
Response: { success: boolean }
```

**Use Cases:**

- Staff-specific permission management
- Integration with HR workflows
- Departmental access control

---

## 🟢 Advanced Features

### 5. Permission Audit & Compliance

#### 5.1 Permission Change Audit Log

```
GET /admin/audit/permissions
Query Parameters:
  - user_id?: string
  - permission?: PermissionEnum
  - action?: 'assigned' | 'unassigned'
  - from?: string
  - to?: string
  - changed_by?: string

Response: {
  data: Array<{
    id: string
    user_id: string
    permission: PermissionEnum
    action: 'assigned' | 'unassigned'
    changed_by: string
    changed_by_email: string
    reason?: string
    created_at: string
  }>
  total: number
}
```

**Use Cases:**

- Security compliance
- Change tracking
- Incident investigation
- SOX/GDPR compliance

#### 5.2 Permission Usage Analytics

```
GET /admin/analytics/permissions/usage
Response: {
  most_assigned: Array<{ permission: PermissionEnum, count: number }>
  least_assigned: Array<{ permission: PermissionEnum, count: number }>
  users_with_most_permissions: Array<{ user_id: string, email: string, count: number }>
  permission_sets_usage: Array<{ set_id: string, name: string, member_count: number }>
}
```

### 6. Advanced Filtering & Search

#### 6.1 Role-Based User Filtering

**Current State:** Users can only be filtered by status, auth method, and date range.

**Recommended Enhancement:**

```
GET /admin/users
Query Parameters (additional):
  - has_role?: RoleEnum
  - has_permission?: PermissionEnum
  - in_permission_set?: string (user_set_id)
  - in_role_set?: string (role_set_id)
```

#### 6.2 Advanced Permission Search

```
GET /admin/permissions/search
Query Parameters:
  - resource?: string (e.g., 'User', 'Role', 'Student')
  - action?: string (e.g., 'Create', 'Read', 'Update', 'Delete')
  - contains?: string (free text search)

Response: {
  permissions: Array<PermissionEnum>
  categories: Array<{ name: string, count: number }>
}
```

### 7. Bulk Operations

#### 7.1 Bulk Permission Assignment

```
POST /admin/users/bulk/permissions
Body: {
  user_ids: string[]
  permissions: PermissionEnum[]
  action: 'assign' | 'unassign'
}
Response: {
  success_count: number
  failed_count: number
  failures: Array<{ user_id: string, error: string }>
}
```

#### 7.2 Bulk Permission Set Assignment

```
POST /admin/permission-sets/bulk/members
Body: {
  permission_set_id: string
  user_ids: string[]
}
Response: {
  added_count: number
  failed_count: number
}
```

### 8. Permission Validation & Safety

#### 8.1 Permission Conflict Detection

```
POST /admin/permissions/validate
Body: {
  user_id: string
  permissions_to_add: PermissionEnum[]
}
Response: {
  conflicts: Array<{
    permission: PermissionEnum
    conflicts_with: PermissionEnum
    reason: string
  }>
  warnings: Array<{
    permission: PermissionEnum
    message: string
  }>
}
```

**Use Cases:**

- Prevent contradictory permissions
- Warn about overly permissive combinations
- Security best practices

#### 8.2 Permission Dependency Analysis

Some permissions may require other permissions to function properly.

```
GET /admin/permissions/{permission}/dependencies
Response: {
  required_permissions: PermissionEnum[]
  recommended_permissions: PermissionEnum[]
}
```

---

## 📋 Implementation Priority

### Phase 1 (High Priority - Security & Operations)

1. ✅ User lock/unlock API (already partially implemented)
2. 🔲 User security settings management
3. 🔲 User activity/audit logs
4. 🔲 Permission change audit log
5. 🔲 User export functionality

### Phase 2 (Medium Priority - UX Improvements)

1. 🔲 User import wizard with preview
2. 🔲 Permission set templates
3. 🔲 Staff permission set UI
4. 🔲 Role set assignment to users
5. 🔲 Advanced filtering (role-based, permission-based)

### Phase 3 (Low Priority - Advanced Features)

1. 🔲 Permission set cloning
2. 🔲 Role set hierarchy
3. 🔲 Permission usage analytics
4. 🔲 Bulk permission operations
5. 🔲 Permission validation & conflict detection

---

## 🔧 Technical Recommendations

### 1. Database Indexing

Ensure the following indexes exist for performance:

```sql
-- User permissions
CREATE INDEX idx_user_permissions_user_id ON user_permissions(user_id);
CREATE INDEX idx_user_permissions_permission ON user_permissions(permission);

-- Permission sets
CREATE INDEX idx_user_set_users_user_set_id ON user_set_users(user_set_id);
CREATE INDEX idx_user_set_users_user_id ON user_set_users(user_id);

-- Role sets
CREATE INDEX idx_role_set_roles_role_set_id ON role_set_roles(role_set_id);
CREATE INDEX idx_role_set_roles_role_id ON role_set_roles(role_id);

-- Audit logs
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
```

### 2. Caching Strategy

- Cache permission lookups for authenticated users (Redis, 5-15 min TTL)
- Cache role set and permission set memberships
- Invalidate cache on permission changes

### 3. Rate Limiting

Implement rate limiting on:

- Permission assignment endpoints (prevent abuse)
- User import/export (resource-intensive)
- Bulk operations

### 4. Event Streaming

Consider implementing event streaming for:

- Permission changes (for real-time audit logs)
- User activity tracking
- Security alerts (e.g., multiple failed login attempts)

---

## 📝 Notes

- All new endpoints should follow existing REST conventions
- Response formats should match existing API patterns
- Include proper error handling with descriptive error messages
- Add OpenAPI/Swagger documentation for all new endpoints
- Implement proper authorization checks (admin-only endpoints)
- Consider pagination for list endpoints
- Add comprehensive input validation

---

## 🚀 Quick Wins

These improvements can be implemented quickly with high impact:

1. **User Export (CSV/JSON)** - 2-4 hours
2. **Permission Set Cloning** - 1-2 hours
3. **User Activity Logs (basic)** - 4-6 hours
4. **Role-Based User Filtering** - 2-3 hours
5. **Permission Change Audit** - 4-6 hours

Total estimated time: ~14-21 hours

---

_Last updated: 2026-03-15_
_Generated as part of RBAC & Users management enhancement_
