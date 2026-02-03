// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Text,
        permission_id -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Text,
        name -> Text,
        parent_id -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Text,
        refresh_token_hash -> Text,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    staff (id) {
        id -> Text,
        employee_id -> Text,
        name -> Text,
        nic -> Text,
        dob -> Date,
        gender -> Text,
        address -> Text,
        phone -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        employment_status -> Text,
        staff_type -> Text,
        photo_url -> Nullable<Text>,
    }
}

diesel::table! {
    staff_attendance (id) {
        id -> Text,
        staff_id -> Text,
        date -> Date,
        status -> Text,
        time_in -> Nullable<Time>,
        time_out -> Nullable<Time>,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_departments (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_employment_history (id) {
        id -> Text,
        staff_id -> Text,
        previous_school -> Text,
        position -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        reason_for_leaving -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_leaves (id) {
        id -> Text,
        staff_id -> Text,
        leave_type -> Text,
        from_date -> Date,
        to_date -> Date,
        reason -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_qualifications (id) {
        id -> Text,
        staff_id -> Text,
        degree -> Text,
        institution -> Text,
        year_of_completion -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_roles (staff_id, role_id) {
        staff_id -> Text,
        role_id -> Text,
    }
}

diesel::table! {
    staff_subjects (staff_id, subject_id) {
        staff_id -> Text,
        subject_id -> Text,
    }
}

diesel::table! {
    teacher_class_assignments (id) {
        id -> Text,
        teacher_id -> Text,
        class_id -> Text,
        academic_year_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teacher_subject_assignments (id) {
        id -> Text,
        teacher_id -> Text,
        subject_id -> Text,
        academic_year_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Text,
        role_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        password_hash -> Text,
        google_id -> Nullable<Text>,
        github_id -> Nullable<Text>,
        is_verified -> Bool,
        verification_token -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        verification_sent_at -> Nullable<Timestamp>,
        password_reset_token -> Nullable<Text>,
        password_reset_sent_at -> Nullable<Timestamp>,
        failed_login_attempts -> Integer,
        lockout_until -> Nullable<Timestamp>,
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(staff_attendance -> staff (staff_id));
diesel::joinable!(staff_employment_history -> staff (staff_id));
diesel::joinable!(staff_leaves -> staff (staff_id));
diesel::joinable!(staff_qualifications -> staff (staff_id));
diesel::joinable!(staff_roles -> roles (role_id));
diesel::joinable!(staff_roles -> staff (staff_id));
diesel::joinable!(staff_subjects -> staff (staff_id));
diesel::joinable!(teacher_class_assignments -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> staff (teacher_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    sessions,
    staff,
    staff_attendance,
    staff_departments,
    staff_employment_history,
    staff_leaves,
    staff_qualifications,
    staff_roles,
    staff_subjects,
    teacher_class_assignments,
    teacher_subject_assignments,
    user_roles,
    users,
);
