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
    al_exams (id) {
        id -> Text,
        student_id -> Text,
        exam_year -> Integer,
        index_number -> Nullable<Text>,
        stream_id -> Nullable<Text>,
        z_score -> Nullable<Double>,
        district_rank -> Nullable<Integer>,
        island_rank -> Nullable<Integer>,
        general_test_marks -> Nullable<Integer>,
        results_summary -> Nullable<Text>,
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
    exam_subjects (exam_id, subject_id) {
        exam_id -> Text,
        subject_id -> Text,
        date -> Date,
        time -> Time,
        duration -> Integer,
        max_marks -> Integer,
        pass_marks -> Integer,
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
    exams (id) {
        id -> Text,
        exam_type_id -> Text,
        name -> Text,
        academic_year_id -> Text,
        term_id -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    grade_levels (id) {
        id -> Text,
        grade_number -> Integer,
        grade_name -> Text,
        education_level -> Text,
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
    grading_criteria (id) {
        id -> Text,
        scheme_id -> Text,
        min_marks -> Integer,
        max_marks -> Integer,
        grade -> Text,
        grade_point -> Nullable<Float>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    grading_schemes (id) {
        id -> Text,
        name -> Text,
        grade_level -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    ol_exams (id) {
        id -> Text,
        student_id -> Text,
        exam_year -> Integer,
        index_number -> Nullable<Text>,
        medium -> Nullable<Text>,
        results_summary -> Nullable<Text>,
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
    report_card_marks (id) {
        id -> Text,
        report_card_id -> Text,
        subject_id -> Text,
        marks_obtained -> Nullable<Integer>,
        grade -> Nullable<Text>,
        remarks -> Nullable<Text>,
    }
}

diesel::table! {
    report_cards (id) {
        id -> Text,
        student_id -> Text,
        academic_year_id -> Text,
        term_id -> Text,
        class_id -> Text,
        generated_at -> Timestamp,
        generated_by -> Text,
        final_grade -> Nullable<Text>,
        rank -> Nullable<Integer>,
        remarks -> Nullable<Text>,
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
    scholarship_exams (id) {
        id -> Text,
        student_id -> Text,
        exam_year -> Integer,
        index_number -> Nullable<Text>,
        marks -> Nullable<Integer>,
        district_rank -> Nullable<Integer>,
        island_rank -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    stream_subjects (stream_id, subject_id) {
        stream_id -> Text,
        subject_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    student_marks (id) {
        id -> Text,
        student_id -> Text,
        exam_id -> Text,
        subject_id -> Text,
        marks_obtained -> Integer,
        is_absent -> Bool,
        remarks -> Nullable<Text>,
        entered_by -> Text,
        entered_at -> Timestamp,
        updated_by -> Nullable<Text>,
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
    student_zscores (student_id, exam_id, subject_id) {
        student_id -> Text,
        exam_id -> Text,
        subject_id -> Text,
        zscore -> Double,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Text,
        photo_url -> Nullable<Text>,
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
    terms (id) {
        id -> Text,
        academic_year_id -> Text,
        term_number -> Integer,
        name -> Text,
        start_date -> Date,
        end_date -> Date,
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

diesel::table! {
    zscore_calculations (exam_id, subject_id) {
        exam_id -> Text,
        subject_id -> Text,
        mean -> Double,
        std_deviation -> Double,
        calculated_at -> Timestamp,
    }
}

diesel::joinable!(al_exams -> streams (stream_id));
diesel::joinable!(al_exams -> students (student_id));
diesel::joinable!(class_subject_teachers -> academic_years (academic_year_id));
diesel::joinable!(class_subject_teachers -> classes (class_id));
diesel::joinable!(class_subject_teachers -> staff (teacher_id));
diesel::joinable!(class_subject_teachers -> subjects (subject_id));
diesel::joinable!(classes -> academic_years (academic_year_id));
diesel::joinable!(classes -> grade_levels (grade_id));
diesel::joinable!(classes -> staff (class_teacher_id));
diesel::joinable!(exam_subjects -> exams (exam_id));
diesel::joinable!(exam_subjects -> subjects (subject_id));
diesel::joinable!(exams -> academic_years (academic_year_id));
diesel::joinable!(exams -> exam_types (exam_type_id));
diesel::joinable!(exams -> terms (term_id));
diesel::joinable!(grade_streams -> grade_levels (grade_id));
diesel::joinable!(grade_streams -> streams (stream_id));
diesel::joinable!(grade_subjects -> grade_levels (grade_id));
diesel::joinable!(grade_subjects -> subjects (subject_id));
diesel::joinable!(grading_criteria -> grading_schemes (scheme_id));
diesel::joinable!(ol_exams -> students (student_id));
diesel::joinable!(report_card_marks -> report_cards (report_card_id));
diesel::joinable!(report_card_marks -> subjects (subject_id));
diesel::joinable!(report_cards -> academic_years (academic_year_id));
diesel::joinable!(report_cards -> classes (class_id));
diesel::joinable!(report_cards -> students (student_id));
diesel::joinable!(report_cards -> terms (term_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(scholarship_exams -> students (student_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(staff_attendance -> staff (staff_id));
diesel::joinable!(staff_employment_history -> staff (staff_id));
diesel::joinable!(staff_leaves -> staff (staff_id));
diesel::joinable!(staff_qualifications -> staff (staff_id));
diesel::joinable!(staff_roles -> roles (role_id));
diesel::joinable!(staff_roles -> staff (staff_id));
diesel::joinable!(staff_subjects -> staff (staff_id));
diesel::joinable!(staff_subjects -> subjects (subject_id));
diesel::joinable!(stream_subjects -> streams (stream_id));
diesel::joinable!(stream_subjects -> subjects (subject_id));
diesel::joinable!(student_attendance -> classes (class_id));
diesel::joinable!(student_attendance -> students (student_id));
diesel::joinable!(student_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(student_class_assignments -> classes (class_id));
diesel::joinable!(student_class_assignments -> grade_levels (grade_id));
diesel::joinable!(student_class_assignments -> students (student_id));
diesel::joinable!(student_emergency_contacts -> students (student_id));
diesel::joinable!(student_guardians -> students (student_id));
diesel::joinable!(student_marks -> students (student_id));
diesel::joinable!(student_medical_info -> students (student_id));
diesel::joinable!(student_previous_schools -> students (student_id));
diesel::joinable!(student_zscores -> students (student_id));
diesel::joinable!(teacher_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(teacher_class_assignments -> classes (class_id));
diesel::joinable!(teacher_class_assignments -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> academic_years (academic_year_id));
diesel::joinable!(teacher_subject_assignments -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> subjects (subject_id));
diesel::joinable!(terms -> academic_years (academic_year_id));
diesel::joinable!(timetable -> academic_years (academic_year_id));
diesel::joinable!(timetable -> classes (class_id));
diesel::joinable!(timetable -> staff (teacher_id));
diesel::joinable!(timetable -> subjects (subject_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    academic_years,
    al_exams,
    class_subject_teachers,
    classes,
    exam_subjects,
    exam_types,
    exams,
    grade_levels,
    grade_streams,
    grade_subjects,
    grading_criteria,
    grading_schemes,
    ol_exams,
    permissions,
    report_card_marks,
    report_cards,
    role_permissions,
    roles,
    scholarship_exams,
    sessions,
    staff,
    staff_attendance,
    staff_departments,
    staff_employment_history,
    staff_leaves,
    staff_qualifications,
    staff_roles,
    staff_subjects,
    stream_subjects,
    streams,
    student_attendance,
    student_class_assignments,
    student_emergency_contacts,
    student_guardians,
    student_marks,
    student_medical_info,
    student_previous_schools,
    student_zscores,
    students,
    subjects,
    teacher_class_assignments,
    teacher_subject_assignments,
    terms,
    timetable,
    user_roles,
    users,
    zscore_calculations,
);
