// @generated automatically by Diesel CLI.

diesel::table! {
    academic_years (id) {
        id -> Text,
        year_start -> Integer,
        year_end -> Integer,
        name -> Text,
        current -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    classes (id) {
        id -> Text,
        grade_id -> Text,
        section_name -> Text,
        academic_year_id -> Text,
        class_teacher_id -> Nullable<Text>,
        medium -> Text,
        room_number -> Nullable<Text>,
        max_capacity -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    class_subject_teachers (class_id, subject_id, teacher_id, academic_year_id) {
        class_id -> Text,
        subject_id -> Text,
        teacher_id -> Text,
        academic_year_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exam_types (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        weightage -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    grade_levels (id) {
        id -> Text,
        grade_number -> Integer,
        grade_name -> Text,
        education_level -> Text, // Store as Text, map to enum in Rust
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    grade_streams (grade_id, stream_id) {
        grade_id -> Text,
        stream_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    grade_subjects (grade_id, subject_id) {
        grade_id -> Text,
        subject_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
        photo_url -> Nullable<Text>,
        employment_status -> Text,
        staff_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    streams (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stream_subjects (stream_id, subject_id) {
        stream_id -> Text,
        subject_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subjects (id) {
        id -> Text,
        subject_code -> Text,
        subject_name_en -> Text,
        subject_name_si -> Nullable<Text>,
        subject_name_ta -> Nullable<Text>,
        is_core -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_attendance (id) {
        id -> Text,
        student_id -> Text,
        class_id -> Text,
        date -> Date,
        status -> Text,
        marked_by -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_class_assignments (id) {
        id -> Text,
        student_id -> Text,
        academic_year_id -> Text,
        grade_id -> Text,
        class_id -> Text,
        from_date -> Date,
        to_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_emergency_contacts (id) {
        id -> Text,
        student_id -> Text,
        name -> Text,
        relationship -> Text,
        phone -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_guardians (id) {
        id -> Text,
        student_id -> Text,
        name -> Text,
        relationship -> Text,
        phone -> Text,
        email -> Nullable<Text>,
        address -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_medical_info (id) {
        id -> Text,
        student_id -> Text,
        blood_group -> Nullable<Text>,
        allergies -> Nullable<Text>,
        medical_conditions -> Nullable<Text>,
        emergency_contact_name -> Nullable<Text>,
        emergency_contact_phone -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_previous_schools (id) {
        id -> Text,
        student_id -> Text,
        school_name -> Text,
        grade_left -> Nullable<Text>,
        date_left -> Nullable<Date>,
        reason_for_leaving -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    students (id) {
        id -> Text,
        admission_number -> Text,
        name_english -> Text,
        name_sinhala -> Nullable<Text>,
        name_tamil -> Nullable<Text>,
        nic_or_birth_certificate -> Text,
        dob -> Date,
        gender -> Text,
        address -> Text,
        phone -> Text,
        email -> Nullable<Text>,
        religion -> Nullable<Text>,
        ethnicity -> Nullable<Text>,
        status -> Text,
        photo_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    timetable (id) {
        id -> Text,
        class_id -> Text,
        day_of_week -> Text,
        period_number -> Integer,
        subject_id -> Text,
        teacher_id -> Text,
        start_time -> Time,
        end_time -> Time,
        room -> Text,
        academic_year_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        verification_sent_at -> Nullable<Timestamp>,
        password_reset_token -> Nullable<Text>,
        password_reset_sent_at -> Nullable<Timestamp>,
        failed_login_attempts -> Integer,
        lockout_until -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(classes -> academic_years (academic_year_id));
diesel::joinable!(classes -> grade_levels (grade_id));
diesel::joinable!(classes -> staff (class_teacher_id));
diesel::joinable!(class_subject_teachers -> academic_years (academic_year_id));
diesel::joinable!(class_subject_teachers -> classes (class_id));
diesel::joinable!(class_subject_teachers -> staff (teacher_id));
diesel::joinable!(class_subject_teachers -> subjects (subject_id));
diesel::joinable!(grade_streams -> grade_levels (grade_id));
diesel::joinable!(grade_streams -> streams (stream_id));
diesel::joinable!(grade_subjects -> grade_levels (grade_id));
diesel::joinable!(grade_subjects -> subjects (subject_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(staff_employment_history -> staff (staff_id));
diesel::joinable!(staff_leaves -> staff (staff_id));
diesel::joinable!(staff_qualifications -> staff (staff_id));
diesel::joinable!(staff_roles -> roles (role_id));
diesel::joinable!(staff_roles -> staff (staff_id));
diesel::joinable!(staff_subjects -> staff (staff_id));
diesel::joinable!(stream_subjects -> streams (stream_id));
diesel::joinable!(stream_subjects -> subjects (subject_id));
diesel::joinable!(student_attendance -> students (student_id));
diesel::joinable!(student_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(student_class_assignments -> grade_levels (grade_id));
diesel::joinable!(student_class_assignments -> students (student_id));
diesel::joinable!(student_emergency_contacts -> students (student_id));
diesel::joinable!(student_guardians -> students (student_id));
diesel::joinable!(student_medical_info -> students (student_id));
diesel::joinable!(student_previous_schools -> students (student_id));
diesel::joinable!(teacher_class_assignments -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> staff (teacher_id));
diesel::joinable!(timetable -> academic_years (academic_year_id));
diesel::joinable!(timetable -> classes (class_id));
diesel::joinable!(timetable -> staff (teacher_id));
diesel::joinable!(timetable -> subjects (subject_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    academic_years,
    classes,
    class_subject_teachers,
    exam_types, // Added this
    grade_levels,
    grade_streams,
    grade_subjects,
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
    streams,
    stream_subjects,
    subjects,
    student_attendance,
    student_class_assignments,
    student_emergency_contacts,
    student_guardians,
    student_medical_info,
    student_previous_schools,
    students,
    teacher_class_assignments,
    teacher_subject_assignments,
    timetable,
    user_roles,
    users,
);