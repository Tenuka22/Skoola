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
    asset_allocations (id) {
        id -> Text,
        item_id -> Text,
        allocated_to_type -> Text,
        allocated_to_id -> Text,
        quantity -> Integer,
        allocation_date -> Timestamp,
        return_date -> Nullable<Timestamp>,
        allocated_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    asset_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    budget_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    budgets (id) {
        id -> Text,
        academic_year_id -> Text,
        category_id -> Text,
        allocated_amount -> Float,
        spent_amount -> Float,
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
    club_activities (id) {
        id -> Text,
        club_id -> Text,
        activity_name -> Text,
        activity_date -> Timestamp,
        description -> Nullable<Text>,
        participants_count -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    club_members (club_id, student_id) {
        club_id -> Text,
        student_id -> Text,
        role -> Text,
        joined_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    clubs (id) {
        id -> Text,
        club_name -> Text,
        description -> Nullable<Text>,
        teacher_in_charge_id -> Text,
        meeting_schedule -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    competition_participants (competition_id, student_id) {
        competition_id -> Text,
        student_id -> Text,
        position -> Nullable<Text>,
        award -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    competitions (id) {
        id -> Text,
        competition_name -> Text,
        competition_type -> Text,
        date -> Timestamp,
        organizer -> Text,
        level -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cultural_event_participants (event_id, student_id) {
        event_id -> Text,
        student_id -> Text,
        performance_type -> Text,
        role -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cultural_events (id) {
        id -> Text,
        event_name -> Text,
        event_date -> Timestamp,
        venue -> Text,
        description -> Nullable<Text>,
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
    expense_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    expense_transactions (id) {
        id -> Text,
        category_id -> Text,
        amount -> Float,
        date -> Timestamp,
        description -> Nullable<Text>,
        vendor -> Nullable<Text>,
        payment_method -> Text,
        approved_by -> Nullable<Text>,
        receipt_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_categories (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        is_mandatory -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_payments (id) {
        id -> Text,
        student_fee_id -> Text,
        amount_paid -> Float,
        payment_date -> Timestamp,
        payment_method -> Text,
        receipt_number -> Text,
        collected_by -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_structures (id) {
        id -> Text,
        grade_id -> Text,
        academic_year_id -> Text,
        category_id -> Text,
        amount -> Float,
        due_date -> Date,
        frequency -> Text,
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
    income_sources (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    income_transactions (id) {
        id -> Text,
        source_id -> Text,
        amount -> Float,
        date -> Timestamp,
        description -> Nullable<Text>,
        received_by -> Text,
        receipt_number -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inventory_items (id) {
        id -> Text,
        category_id -> Text,
        item_name -> Text,
        description -> Nullable<Text>,
        unit -> Text,
        quantity -> Integer,
        reorder_level -> Integer,
        unit_price -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    library_books (id) {
        id -> Integer,
        isbn -> Nullable<Text>,
        title -> Text,
        author -> Text,
        publisher -> Nullable<Text>,
        category_id -> Integer,
        quantity -> Integer,
        available_quantity -> Integer,
        rack_number -> Nullable<Text>,
        added_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    library_categories (id) {
        id -> Integer,
        category_name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    library_issues (id) {
        id -> Integer,
        book_id -> Integer,
        student_id -> Nullable<Text>,
        staff_id -> Nullable<Text>,
        issue_date -> Date,
        due_date -> Date,
        return_date -> Nullable<Date>,
        issued_by -> Text,
        fine_amount -> Nullable<Float>,
        fine_paid -> Bool,
        status -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    library_settings (id) {
        id -> Integer,
        max_books_per_student -> Integer,
        max_books_per_staff -> Integer,
        issue_duration_days_student -> Integer,
        issue_duration_days_staff -> Integer,
        fine_per_day -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    maintenance_requests (id) {
        id -> Text,
        item_id -> Text,
        issue_description -> Text,
        reported_by -> Text,
        reported_date -> Timestamp,
        status -> Text,
        assigned_to -> Nullable<Text>,
        resolved_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    petty_cash_transactions (id) {
        id -> Text,
        amount -> Float,
        transaction_type -> Text,
        date -> Timestamp,
        description -> Nullable<Text>,
        handled_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    salary_components (id) {
        id -> Text,
        name -> Text,
        component_type -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    salary_payments (id) {
        id -> Text,
        staff_id -> Text,
        payment_month -> Integer,
        payment_year -> Integer,
        gross_salary -> Float,
        total_deductions -> Float,
        net_salary -> Float,
        payment_date -> Timestamp,
        payment_method -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    sport_event_participants (event_id, student_id) {
        event_id -> Text,
        student_id -> Text,
        team_id -> Nullable<Text>,
        position -> Nullable<Text>,
        points -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sport_events (id) {
        id -> Text,
        sport_id -> Text,
        event_name -> Text,
        event_date -> Timestamp,
        venue -> Text,
        organizer -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sport_team_members (team_id, student_id) {
        team_id -> Text,
        student_id -> Text,
        position -> Nullable<Text>,
        joined_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sport_teams (id) {
        id -> Text,
        sport_id -> Text,
        team_name -> Text,
        grade_level -> Text,
        coach_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sports (id) {
        id -> Text,
        sport_name -> Text,
        description -> Nullable<Text>,
        category -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    staff_salaries (staff_id, component_id) {
        staff_id -> Text,
        component_id -> Text,
        amount -> Float,
        effective_from -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    student_achievements (id) {
        id -> Text,
        student_id -> Text,
        achievement_type -> Text,
        description -> Text,
        date -> Date,
        certificate_url -> Nullable<Text>,
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
    student_fees (id) {
        id -> Text,
        student_id -> Text,
        fee_structure_id -> Text,
        amount -> Float,
        is_exempted -> Bool,
        exemption_reason -> Nullable<Text>,
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
    uniform_issues (id) {
        id -> Text,
        student_id -> Text,
        uniform_item_id -> Text,
        quantity -> Integer,
        issue_date -> Timestamp,
        issued_by -> Text,
        amount_collected -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    uniform_items (id) {
        id -> Text,
        item_name -> Text,
        size -> Text,
        gender -> Text,
        grade_level -> Nullable<Text>,
        price -> Float,
        quantity -> Integer,
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
diesel::joinable!(asset_allocations -> inventory_items (item_id));
diesel::joinable!(asset_allocations -> staff (allocated_by));
diesel::joinable!(budgets -> academic_years (academic_year_id));
diesel::joinable!(budgets -> budget_categories (category_id));
diesel::joinable!(class_subject_teachers -> academic_years (academic_year_id));
diesel::joinable!(class_subject_teachers -> classes (class_id));
diesel::joinable!(class_subject_teachers -> staff (teacher_id));
diesel::joinable!(class_subject_teachers -> subjects (subject_id));
diesel::joinable!(classes -> academic_years (academic_year_id));
diesel::joinable!(classes -> grade_levels (grade_id));
diesel::joinable!(classes -> staff (class_teacher_id));
diesel::joinable!(club_activities -> clubs (club_id));
diesel::joinable!(club_members -> clubs (club_id));
diesel::joinable!(club_members -> students (student_id));
diesel::joinable!(clubs -> staff (teacher_in_charge_id));
diesel::joinable!(competition_participants -> competitions (competition_id));
diesel::joinable!(competition_participants -> students (student_id));
diesel::joinable!(cultural_event_participants -> cultural_events (event_id));
diesel::joinable!(cultural_event_participants -> students (student_id));
diesel::joinable!(exam_subjects -> exams (exam_id));
diesel::joinable!(exam_subjects -> subjects (subject_id));
diesel::joinable!(exams -> academic_years (academic_year_id));
diesel::joinable!(exams -> exam_types (exam_type_id));
diesel::joinable!(exams -> terms (term_id));
diesel::joinable!(expense_transactions -> expense_categories (category_id));
diesel::joinable!(expense_transactions -> staff (approved_by));
diesel::joinable!(fee_payments -> staff (collected_by));
diesel::joinable!(fee_payments -> student_fees (student_fee_id));
diesel::joinable!(fee_structures -> academic_years (academic_year_id));
diesel::joinable!(fee_structures -> fee_categories (category_id));
diesel::joinable!(fee_structures -> grade_levels (grade_id));
diesel::joinable!(grade_streams -> grade_levels (grade_id));
diesel::joinable!(grade_streams -> streams (stream_id));
diesel::joinable!(grade_subjects -> grade_levels (grade_id));
diesel::joinable!(grade_subjects -> subjects (subject_id));
diesel::joinable!(grading_criteria -> grading_schemes (scheme_id));
diesel::joinable!(income_transactions -> income_sources (source_id));
diesel::joinable!(income_transactions -> staff (received_by));
diesel::joinable!(inventory_items -> asset_categories (category_id));
diesel::joinable!(library_books -> library_categories (category_id));
diesel::joinable!(library_issues -> library_books (book_id));
diesel::joinable!(library_issues -> students (student_id));
diesel::joinable!(maintenance_requests -> inventory_items (item_id));
diesel::joinable!(maintenance_requests -> staff (reported_by));
diesel::joinable!(ol_exams -> students (student_id));
diesel::joinable!(petty_cash_transactions -> staff (handled_by));
diesel::joinable!(report_card_marks -> report_cards (report_card_id));
diesel::joinable!(report_card_marks -> subjects (subject_id));
diesel::joinable!(report_cards -> academic_years (academic_year_id));
diesel::joinable!(report_cards -> classes (class_id));
diesel::joinable!(report_cards -> students (student_id));
diesel::joinable!(report_cards -> terms (term_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(salary_payments -> staff (staff_id));
diesel::joinable!(scholarship_exams -> students (student_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(sport_event_participants -> sport_events (event_id));
diesel::joinable!(sport_event_participants -> sport_teams (team_id));
diesel::joinable!(sport_event_participants -> students (student_id));
diesel::joinable!(sport_events -> sports (sport_id));
diesel::joinable!(sport_team_members -> sport_teams (team_id));
diesel::joinable!(sport_team_members -> students (student_id));
diesel::joinable!(sport_teams -> sports (sport_id));
diesel::joinable!(sport_teams -> staff (coach_id));
diesel::joinable!(staff_attendance -> staff (staff_id));
diesel::joinable!(staff_employment_history -> staff (staff_id));
diesel::joinable!(staff_leaves -> staff (staff_id));
diesel::joinable!(staff_qualifications -> staff (staff_id));
diesel::joinable!(staff_roles -> roles (role_id));
diesel::joinable!(staff_roles -> staff (staff_id));
diesel::joinable!(staff_salaries -> salary_components (component_id));
diesel::joinable!(staff_salaries -> staff (staff_id));
diesel::joinable!(staff_subjects -> staff (staff_id));
diesel::joinable!(staff_subjects -> subjects (subject_id));
diesel::joinable!(stream_subjects -> streams (stream_id));
diesel::joinable!(stream_subjects -> subjects (subject_id));
diesel::joinable!(student_achievements -> students (student_id));
diesel::joinable!(student_emergency_contacts -> students (student_id));
diesel::joinable!(student_fees -> fee_structures (fee_structure_id));
diesel::joinable!(student_fees -> students (student_id));
diesel::joinable!(student_guardians -> students (student_id));
diesel::joinable!(student_marks -> students (student_id));
diesel::joinable!(student_medical_info -> students (student_id));
diesel::joinable!(student_previous_schools -> students (student_id));
diesel::joinable!(student_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(student_class_assignments -> classes (class_id));
diesel::joinable!(student_class_assignments -> students (student_id));
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
diesel::joinable!(uniform_issues -> staff (issued_by));
diesel::joinable!(uniform_issues -> students (student_id));
diesel::joinable!(uniform_issues -> uniform_items (uniform_item_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    academic_years,
    al_exams,
    asset_allocations,
    asset_categories,
    budget_categories,
    budgets,
    class_subject_teachers,
    classes,
    club_activities,
    club_members,
    clubs,
    competition_participants,
    competitions,
    cultural_event_participants,
    cultural_events,
    exam_subjects,
    exam_types,
    exams,
    expense_categories,
    expense_transactions,
    fee_categories,
    fee_payments,
    fee_structures,
    grade_levels,
    grade_streams,
    grade_subjects,
    grading_criteria,
    grading_schemes,
    income_sources,
    income_transactions,
    inventory_items,
    library_books,
    library_categories,
    library_issues,
    library_settings,
    maintenance_requests,
    ol_exams,
    permissions,
    petty_cash_transactions,
    report_card_marks,
    report_cards,
    role_permissions,
    roles,
    salary_components,
    salary_payments,
    scholarship_exams,
    sessions,
    sport_event_participants,
    sport_events,
    sport_team_members,
    sport_teams,
    sports,
    staff,
    staff_attendance,
    staff_departments,
    staff_employment_history,
    staff_leaves,
    staff_qualifications,
    staff_roles,
    staff_salaries,
    staff_subjects,
    stream_subjects,
    streams,
    student_achievements,
    student_attendance,
    student_class_assignments,
    student_emergency_contacts,
    student_fees,
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
    uniform_issues,
    uniform_items,
    user_roles,
    users,
    zscore_calculations,
);
