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
    activities (id) {
        id -> Text,
        activity_type_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        location -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        is_mandatory -> Bool,
        academic_year_id -> Text,
        created_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    activity_attendance (id) {
        id -> Text,
        activity_id -> Text,
        user_id -> Text,
        status -> Text,
        check_in_time -> Nullable<Timestamp>,
        check_out_time -> Nullable<Timestamp>,
        remarks -> Nullable<Text>,
        marked_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    activity_participants (activity_id, user_id) {
        activity_id -> Text,
        user_id -> Text,
        participant_type -> Text,
        enrollment_reason -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    activity_participants_staff (activity_id, staff_id) {
        activity_id -> Text,
        staff_id -> Text,
        participant_type -> Text,
        enrollment_reason -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    activity_participants_students (activity_id, student_id) {
        activity_id -> Text,
        student_id -> Text,
        participant_type -> Text,
        enrollment_reason -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    activity_types (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ai_processed_note_sections (id) {
        id -> Text,
        note_id -> Text,
        section_type -> Text,
        content -> Text,
        order_index -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ai_processed_notes (id) {
        id -> Text,
        material_id -> Text,
        structured_json -> Text,
        summary -> Nullable<Text>,
        key_takeaways -> Nullable<Text>,
        suggested_questions -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    al_stream_grade_levels (stream_id, grade_level_id) {
        stream_id -> Text,
        grade_level_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    al_stream_optional_groups (id) {
        id -> Text,
        stream_id -> Text,
        group_name -> Text,
        min_select -> Integer,
        max_select -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    al_stream_optional_subjects (group_id, subject_id) {
        group_id -> Text,
        subject_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    al_stream_required_subjects (stream_id, subject_id) {
        stream_id -> Text,
        subject_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    al_streams (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        version_name -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        is_active -> Bool,
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
    asset_allocations_staff (asset_allocation_id, staff_id) {
        asset_allocation_id -> Text,
        staff_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    asset_allocations_students (asset_allocation_id, student_id) {
        asset_allocation_id -> Text,
        student_id -> Text,
        created_at -> Timestamp,
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
    asset_maintenance_logs (id) {
        id -> Text,
        item_id -> Text,
        maintenance_date -> Date,
        maintenance_type -> Text,
        notes -> Nullable<Text>,
        cost -> Nullable<Float>,
        performed_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    attendance_audit_log (id) {
        id -> Text,
        attendance_type -> Text,
        attendance_record_id -> Text,
        old_status -> Nullable<Text>,
        new_status -> Text,
        change_reason -> Text,
        changed_by -> Text,
        changed_at -> Timestamp,
    }
}

diesel::table! {
    attendance_discrepancies (id) {
        id -> Text,
        student_id -> Text,
        date -> Date,
        discrepancy_type -> Text,
        details -> Nullable<Text>,
        severity -> Text,
        is_resolved -> Bool,
        resolved_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    attendance_excuses (id) {
        id -> Text,
        attendance_record_id -> Text,
        excuse_type -> Text,
        document_url -> Nullable<Text>,
        is_verified -> Bool,
        verified_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    attendance_policies (id) {
        id -> Text,
        name -> Text,
        rule_type -> Text,
        threshold -> Integer,
        consequence_type -> Text,
        consequence_value -> Nullable<Float>,
        is_active -> Bool,
    }
}

diesel::table! {
    audit_log (id) {
        id -> Text,
        user_id -> Text,
        action_type -> Text,
        table_name -> Text,
        record_pk -> Text,
        old_value_json -> Nullable<Text>,
        new_value_json -> Nullable<Text>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    auth_tokens (id) {
        id -> Text,
        user_id -> Text,
        token_hash -> Text,
        token_type -> Text,
        issued_at -> Timestamp,
        expires_at -> Timestamp,
        revoked_at -> Nullable<Timestamp>,
        is_active -> Bool,
        metadata -> Nullable<Text>,
    }
}

diesel::table! {
    behavior_incident_actions (id) {
        id -> Text,
        incident_id -> Text,
        action_type -> Text,
        action_details -> Nullable<Text>,
        assigned_to -> Nullable<Text>,
        due_date -> Nullable<Date>,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_details (incident_id) {
        incident_id -> Text,
        description -> Text,
        points_awarded -> Integer,
        severity_id -> Nullable<Text>,
        status -> Text,
        resolved_by -> Nullable<Text>,
        resolved_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_evidence (id) {
        id -> Text,
        incident_id -> Text,
        file_url -> Text,
        file_type -> Nullable<Text>,
        uploaded_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_followups (id) {
        id -> Text,
        incident_id -> Text,
        followup_date -> Date,
        notes -> Nullable<Text>,
        recorded_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_participants (incident_id, participant_type, participant_id) {
        incident_id -> Text,
        participant_type -> Text,
        participant_id -> Text,
        role -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_severity_levels (id) {
        id -> Text,
        name -> Text,
        points -> Integer,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incident_types (id) {
        id -> Text,
        type_name -> Text,
        default_points -> Integer,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    behavior_incidents (id) {
        id -> Text,
        student_id -> Text,
        reported_by_user_id -> Text,
        incident_type_id -> Text,
        incident_date -> Timestamp,
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
    chart_of_accounts (id) {
        id -> Text,
        account_code -> Text,
        account_name -> Text,
        account_type -> Text,
        normal_balance -> Text,
        description -> Nullable<Text>,
        parent_account_id -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        currency -> Text,
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
        academic_year_id -> Text,
        class_teacher_id -> Nullable<Text>,
        medium -> Text,
        room_id -> Nullable<Text>,
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
        representing_type -> Nullable<Text>,
        representing_id -> Nullable<Text>,
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
    conversation_participants (conversation_id, user_id) {
        conversation_id -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    conversations (id) {
        id -> Text,
        subject -> Text,
        created_at -> Timestamp,
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
    curriculum_standards (id) {
        id -> Text,
        subject_id -> Text,
        grade_level_id -> Text,
        standard_code -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        medium -> Text,
        version_name -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        is_active -> Bool,
        stream_id -> Nullable<Text>,
    }
}

diesel::table! {
    curriculum_topics (id) {
        id -> Text,
        curriculum_standard_id -> Text,
        parent_id -> Nullable<Text>,
        topic_name -> Text,
        full_time_hours -> Float,
        extra_time_hours -> Float,
        practical_hours -> Float,
        order_index -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    detention_balances (student_id) {
        student_id -> Text,
        total_hours_assigned -> Float,
        total_hours_served -> Float,
        remaining_hours -> Float,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    emergency_roll_call_entries (roll_call_id, user_id) {
        roll_call_id -> Text,
        user_id -> Text,
        status -> Text,
        location_found -> Nullable<Text>,
        marked_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    emergency_roll_calls (id) {
        id -> Text,
        event_name -> Text,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        initiated_by -> Text,
        status -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exam_structure_subjects (id) {
        id -> Text,
        structure_id -> Text,
        subject_id -> Text,
        duration_minutes -> Nullable<Integer>,
        max_marks -> Nullable<Integer>,
        pass_marks -> Nullable<Integer>,
        order_index -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exam_structures (id) {
        id -> Text,
        name -> Text,
        scope_type -> Text,
        medium -> Nullable<Text>,
        description -> Nullable<Text>,
        valid_from -> Nullable<Date>,
        valid_to -> Nullable<Date>,
        is_active -> Bool,
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
        term_id -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exam_subjects (id) {
        id -> Text,
        exam_id -> Text,
        subject_id -> Text,
        date -> Nullable<Date>,
        time -> Nullable<Time>,
        duration -> Nullable<Integer>,
        max_marks -> Nullable<Integer>,
        pass_marks -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exam_types (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        weightage -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exit_passes (id) {
        id -> Text,
        student_id -> Text,
        date -> Date,
        exit_time -> Time,
        reason_type -> Text,
        remarks -> Nullable<Text>,
        approved_by -> Text,
        guardian_notified -> Bool,
        gate_cleared_at -> Nullable<Timestamp>,
        bulk_pass_id -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exit_passes_bulk (id) {
        id -> Text,
        target_type -> Text,
        target_id -> Text,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        reason -> Nullable<Text>,
        issued_by -> Text,
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
    fee_invoice_items (id) {
        id -> Text,
        invoice_id -> Text,
        fee_structure_item_id -> Nullable<Text>,
        description -> Text,
        quantity -> Float,
        unit_amount -> Float,
        total_amount -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_invoices (id) {
        id -> Text,
        student_id -> Text,
        academic_year_id -> Text,
        term_id -> Nullable<Text>,
        status -> Text,
        issued_at -> Nullable<Timestamp>,
        due_date -> Nullable<Date>,
        total_amount -> Float,
        balance_amount -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_payment_allocations (id) {
        id -> Text,
        payment_id -> Text,
        invoice_id -> Text,
        amount -> Float,
        created_at -> Timestamp,
    }
}

diesel::table! {
    fee_payment_details (payment_id) {
        payment_id -> Text,
        payment_method -> Text,
        payment_channel -> Nullable<Text>,
        payment_status -> Text,
        receipt_number -> Text,
        transaction_reference -> Nullable<Text>,
        remarks -> Nullable<Text>,
        recorded_by -> Nullable<Text>,
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
        collected_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_structure_items (id) {
        id -> Text,
        fee_structure_id -> Text,
        item_name -> Text,
        amount -> Float,
        is_optional -> Bool,
        order_index -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_structure_pricing (fee_structure_id) {
        fee_structure_id -> Text,
        amount -> Float,
        currency -> Text,
        amount_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fee_structure_schedule (fee_structure_id) {
        fee_structure_id -> Text,
        due_date -> Nullable<Date>,
        frequency -> Text,
        fee_type -> Text,
        effective_from -> Nullable<Date>,
        effective_to -> Nullable<Date>,
        due_day_of_month -> Nullable<Integer>,
        is_refundable -> Bool,
        late_fee_type -> Nullable<Text>,
        late_fee_value -> Nullable<Float>,
        is_active -> Bool,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    general_ledger (id) {
        id -> Text,
        transaction_date -> Date,
        description -> Nullable<Text>,
        debit_account_id -> Text,
        credit_account_id -> Text,
        amount -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    government_exam_subjects (id) {
        id -> Text,
        government_exam_id -> Text,
        subject_id -> Text,
        exam_date -> Nullable<Date>,
        exam_time -> Nullable<Time>,
        duration_minutes -> Nullable<Integer>,
        max_marks -> Nullable<Integer>,
        pass_marks -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    government_exams (id) {
        id -> Text,
        exam_structure_id -> Text,
        name -> Text,
        authority -> Nullable<Text>,
        level -> Nullable<Text>,
        exam_year -> Nullable<Integer>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        status -> Text,
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
    grade_periods (id) {
        id -> Text,
        grade_id -> Text,
        start_time -> Time,
        end_time -> Time,
        is_break -> Bool,
        is_optional -> Bool,
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
    grading_schemes (id) {
        id -> Text,
        name -> Text,
        scheme_type -> Text,
        grade_level_id -> Nullable<Text>,
        scale_definition -> Text,
        pass_mark -> Nullable<Integer>,
        is_default -> Bool,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    grading_criteria (id) {
        id -> Text,
        scheme_id -> Text,
        grade -> Text,
        min_mark -> Integer,
        max_mark -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    inventory_item_details (item_id) {
        item_id -> Text,
        description -> Nullable<Text>,
        quantity -> Integer,
        reorder_level -> Integer,
        unit_price -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inventory_items (id) {
        id -> Text,
        category_id -> Text,
        item_name -> Text,
        unit -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inventory_transactions (id) {
        id -> Text,
        item_id -> Text,
        transaction_type -> Text,
        quantity -> Float,
        unit_cost -> Nullable<Float>,
        transaction_date -> Timestamp,
        reference_type -> Nullable<Text>,
        reference_id -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ledger_entries (id) {
        id -> Text,
        transaction_id -> Text,
        account_id -> Text,
        entry_type -> Text,
        amount -> Float,
        memo -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ledger_transactions (id) {
        id -> Text,
        transaction_date -> Timestamp,
        description -> Nullable<Text>,
        reference_type -> Nullable<Text>,
        reference_id -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    lesson_materials (id) {
        id -> Text,
        lesson_progress_id -> Text,
        uploader_id -> Text,
        file_name -> Text,
        file_url -> Text,
        file_type -> Text,
        is_processed_by_ai -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    lesson_progress (id) {
        id -> Text,
        class_id -> Text,
        subject_id -> Text,
        teacher_id -> Text,
        timetable_id -> Nullable<Text>,
        curriculum_topic_id -> Nullable<Text>,
        date -> Date,
        lesson_summary -> Text,
        homework_assigned -> Nullable<Text>,
        resources_used -> Nullable<Text>,
        progress_percentage -> Nullable<Integer>,
        delivery_mode -> Text,
        planned_duration_minutes -> Nullable<Integer>,
        actual_duration_minutes -> Nullable<Integer>,
        is_skipped -> Bool,
        priority_level -> Integer,
        verified_by -> Nullable<Text>,
        verified_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    lesson_progress_attachments (id) {
        id -> Text,
        lesson_progress_id -> Text,
        file_name -> Text,
        file_url -> Text,
        file_type -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    lesson_progress_periods (lesson_progress_id, timetable_id, date) {
        lesson_progress_id -> Text,
        timetable_id -> Text,
        date -> Date,
    }
}

diesel::table! {
    lesson_reviews (id) {
        id -> Text,
        lesson_progress_id -> Text,
        reviewer_type -> Text,
        reviewer_id -> Text,
        clarity_rating -> Integer,
        feedback_text -> Nullable<Text>,
        created_at -> Timestamp,
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
    marking_scheme_parts (id) {
        id -> Text,
        scheme_id -> Text,
        paper_label -> Text,
        part_label -> Text,
        question_label -> Nullable<Text>,
        max_marks -> Float,
        weight_ratio -> Nullable<Float>,
        structure_json -> Nullable<Text>,
        order_index -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    marking_schemes (id) {
        id -> Text,
        name -> Text,
        subject_id -> Text,
        grade_level_id -> Nullable<Text>,
        curriculum_standard_id -> Nullable<Text>,
        stream_id -> Nullable<Text>,
        description -> Nullable<Text>,
        valid_from -> Nullable<Date>,
        valid_to -> Nullable<Date>,
        calculation_fn -> Text,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Text,
        conversation_id -> Text,
        sender_user_id -> Text,
        content -> Text,
        sent_at -> Timestamp,
        read_at -> Nullable<Timestamp>,
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
    practical_lesson_appeals (id) {
        id -> Text,
        lesson_progress_id -> Text,
        appeal_reason -> Text,
        evidence_image_url -> Nullable<Text>,
        status -> Text,
        reviewed_by -> Nullable<Text>,
        reviewed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    pre_approved_absences (id) {
        id -> Text,
        student_id -> Text,
        start_date -> Date,
        end_date -> Date,
        reason_type -> Text,
        remarks -> Nullable<Text>,
        approved_by -> Text,
        document_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profile_contacts (profile_id) {
        profile_id -> Text,
        address -> Nullable<Text>,
        phone -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profile_media (profile_id) {
        profile_id -> Text,
        photo_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    purchase_order_items (id) {
        id -> Text,
        purchase_order_id -> Text,
        item_name -> Text,
        quantity -> Float,
        unit_price -> Float,
        total_price -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    purchase_orders (id) {
        id -> Text,
        vendor_id -> Text,
        order_date -> Date,
        status -> Text,
        total_amount -> Float,
        created_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    report_card_marks (id) {
        id -> Text,
        report_card_id -> Text,
        subject_id -> Text,
        assessment_type -> Text,
        assessment_id -> Text,
        marking_scheme_id -> Nullable<Text>,
        total_marks -> Nullable<Float>,
        percentage -> Nullable<Float>,
        grade -> Nullable<Text>,
        grade_point -> Nullable<Float>,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    report_cards (id) {
        id -> Text,
        student_id -> Text,
        academic_year_id -> Text,
        term_id -> Text,
        class_id -> Text,
        grading_scheme_id -> Nullable<Text>,
        generated_at -> Timestamp,
        generated_by -> Text,
        overall_percentage -> Nullable<Float>,
        overall_grade -> Nullable<Text>,
        overall_gpa -> Nullable<Float>,
        rank -> Nullable<Integer>,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    resource_assets (id) {
        id -> Text,
        resource_id -> Text,
        inventory_item_id -> Text,
        quantity -> Float,
        created_at -> Timestamp,
    }
}

diesel::table! {
    resource_bookings (id) {
        id -> Text,
        resource_id -> Text,
        booked_by_user_id -> Text,
        start_time -> Timestamp,
        end_time -> Timestamp,
        related_event_id -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    resource_details (resource_id) {
        resource_id -> Text,
        description -> Nullable<Text>,
        status -> Text,
        location -> Nullable<Text>,
        capacity -> Nullable<Integer>,
        booking_policy -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    resources (id) {
        id -> Text,
        resource_name -> Text,
        resource_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reward_adjustments (id) {
        id -> Text,
        teacher_id -> Text,
        adjustment_points -> Integer,
        reason -> Nullable<Text>,
        approved_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reward_types (id) {
        id -> Text,
        name -> Text,
        category -> Text,
        default_points -> Integer,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    role_permissions (role_id, permission) {
        role_id -> Text,
        permission -> Text,
    }
}

diesel::table! {
    role_set_roles (role_set_id, role_id) {
        role_set_id -> Text,
        role_id -> Text,
    }
}

diesel::table! {
    role_sets (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
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
    school_calendar (date) {
        date -> Date,
        day_type -> Text,
        name -> Nullable<Text>,
        is_academic_day -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    school_rooms (id) {
        id -> Text,
        name -> Nullable<Text>,
        building -> Nullable<Text>,
        floor -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    school_settings (setting_key) {
        setting_key -> Text,
        setting_value -> Text,
        description -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    school_test_subjects (id) {
        id -> Text,
        school_test_id -> Text,
        subject_id -> Text,
        test_date -> Nullable<Date>,
        test_time -> Nullable<Time>,
        duration_minutes -> Nullable<Integer>,
        max_marks -> Nullable<Integer>,
        pass_marks -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    school_tests (id) {
        id -> Text,
        exam_structure_id -> Text,
        name -> Text,
        test_type -> Text,
        academic_year_id -> Text,
        term_id -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        created_by -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    seeds (id) {
        id -> Text,
        table_name -> Text,
        record_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Text,
        auth_token_id -> Nullable<Text>,
        verification_token_id -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        is_active -> Bool,
        disabled_at -> Nullable<Timestamp>,
        disabled_reason -> Nullable<Text>,
        last_seen_at -> Nullable<Timestamp>,
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
        dob -> Date,
        gender -> Text,
        staff_type -> Text,
        profile_id -> Nullable<Text>,
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
        reason_type -> Nullable<Text>,
        reason_details -> Nullable<Text>,
        half_day_type -> Nullable<Text>,
        out_of_school_from -> Nullable<Time>,
        out_of_school_to -> Nullable<Time>,
        attendance_context -> Nullable<Text>,
        event_id -> Nullable<Text>,
        approved_by -> Nullable<Text>,
        approval_status -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_locked -> Bool,
        marked_by -> Nullable<Text>,
    }
}

diesel::table! {
    staff_contacts (staff_id) {
        staff_id -> Text,
        address -> Text,
        phone -> Text,
        email -> Text,
        address_latitude -> Nullable<Float>,
        address_longitude -> Nullable<Float>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_contracts (id) {
        id -> Text,
        staff_id -> Text,
        contract_type -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        salary_amount -> Nullable<Float>,
        currency -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_cvs (id) {
        id -> Text,
        staff_id -> Text,
        file_name -> Text,
        file_url -> Text,
        file_type -> Text,
        uploaded_at -> Timestamp,
        created_at -> Timestamp,
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
    staff_documents (id) {
        id -> Text,
        staff_id -> Text,
        doc_type -> Text,
        file_url -> Text,
        issued_date -> Nullable<Date>,
        expiry_date -> Nullable<Date>,
        created_at -> Timestamp,
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
        workplace_address -> Nullable<Text>,
        workplace_contact_number -> Nullable<Text>,
        workplace_email -> Nullable<Text>,
    }
}

diesel::table! {
    staff_employment_status (staff_id) {
        staff_id -> Text,
        employment_status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_event_participants (event_id, staff_id) {
        event_id -> Text,
        staff_id -> Text,
        participation_status -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    staff_events (id) {
        id -> Text,
        event_name -> Text,
        event_type -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        location -> Nullable<Text>,
        organizer -> Nullable<Text>,
        counts_as_attendance -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_identity (staff_id) {
        staff_id -> Text,
        nic -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_leave_balances (staff_id, leave_type_id) {
        staff_id -> Text,
        leave_type_id -> Text,
        balance_days -> Float,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_leave_requests (id) {
        id -> Text,
        staff_id -> Text,
        leave_type_id -> Text,
        start_date -> Date,
        end_date -> Date,
        reason -> Nullable<Text>,
        status -> Text,
        approved_by -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_leave_types (id) {
        id -> Text,
        name -> Text,
        annual_quota -> Float,
        requires_approval -> Bool,
        created_at -> Timestamp,
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
    staff_media (staff_id) {
        staff_id -> Text,
        photo_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_notes (id) {
        id -> Text,
        staff_id -> Text,
        note_type -> Text,
        note_text -> Text,
        created_by -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    staff_overtime (id) {
        id -> Text,
        staff_id -> Text,
        date -> Date,
        hours -> Float,
        reason -> Nullable<Text>,
        approved_by -> Nullable<Text>,
        reward_points -> Integer,
        is_paid -> Bool,
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
        file_name -> Nullable<Text>,
        file_url -> Nullable<Text>,
        file_type -> Nullable<Text>,
    }
}

diesel::table! {
    staff_reward_snapshots (staff_id) {
        staff_id -> Text,
        reward_points_balance -> Integer,
        updated_at -> Timestamp,
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
    staff_skills (id) {
        id -> Text,
        staff_id -> Text,
        skill_name -> Text,
        proficiency_level -> Text,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    staff_subject_expertise (staff_id, subject_id) {
        staff_id -> Text,
        subject_id -> Text,
        expertise_level -> Text,
        years_experience -> Nullable<Integer>,
        evidence -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    staff_subjects (staff_id, subject_id) {
        staff_id -> Text,
        subject_id -> Text,
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
    student_allergies (id) {
        id -> Text,
        student_id -> Text,
        allergen_type -> Text,
        allergen_name -> Text,
        reaction_severity -> Text,
        reaction_description -> Nullable<Text>,
        requires_epipen -> Bool,
        notes -> Nullable<Text>,
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
        is_locked -> Bool,
    }
}

diesel::table! {
    student_birth_certificates (id) {
        id -> Text,
        student_id -> Text,
        certificate_number -> Text,
        issued_date -> Nullable<Date>,
        document_url -> Nullable<Text>,
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
    student_class_assignments_history (id) {
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
    student_contacts (student_id) {
        student_id -> Text,
        address -> Text,
        address_latitude -> Nullable<Float>,
        address_longitude -> Nullable<Float>,
        phone -> Text,
        email -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_demographics (student_id) {
        student_id -> Text,
        religion -> Nullable<Text>,
        ethnicity -> Nullable<Text>,
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
        user_id -> Nullable<Text>,
    }
}

diesel::table! {
    student_mark_entries (id) {
        id -> Text,
        student_mark_id -> Text,
        marking_scheme_part_id -> Text,
        marks_awarded -> Float,
        max_marks -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_mark_entries_history (id) {
        id -> Text,
        student_marks_history_id -> Text,
        marking_scheme_part_id -> Text,
        marks_awarded -> Float,
        max_marks -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_marks (id) {
        id -> Text,
        student_id -> Text,
        subject_id -> Text,
        assessment_type -> Text,
        assessment_id -> Text,
        marking_scheme_id -> Text,
        total_marks -> Nullable<Float>,
        percentage -> Nullable<Float>,
        grade -> Nullable<Text>,
        grade_point -> Nullable<Float>,
        is_absent -> Bool,
        remarks -> Nullable<Text>,
        entered_by -> Text,
        entered_at -> Timestamp,
        updated_by -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_marks_history (id) {
        id -> Text,
        student_id -> Text,
        subject_id -> Text,
        assessment_type -> Text,
        assessment_id -> Text,
        marking_scheme_id -> Text,
        total_marks -> Nullable<Float>,
        percentage -> Nullable<Float>,
        grade -> Nullable<Text>,
        grade_point -> Nullable<Float>,
        is_absent -> Bool,
        remarks -> Nullable<Text>,
        entered_by -> Text,
        entered_at -> Timestamp,
        updated_by -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_media (student_id) {
        student_id -> Text,
        photo_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_medical_conditions (id) {
        id -> Text,
        student_id -> Text,
        condition_type -> Text,
        condition_name -> Text,
        severity -> Text,
        diagnosis_date -> Nullable<Date>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_medical_info (id) {
        id -> Text,
        student_id -> Text,
        blood_group -> Nullable<Text>,
        medical_risk_level -> Nullable<Text>,
        has_allergies -> Bool,
        has_medications -> Bool,
        has_chronic_conditions -> Bool,
        requires_emergency_plan -> Bool,
        emergency_plan_details -> Nullable<Text>,
        allergies -> Nullable<Text>,
        medical_conditions -> Nullable<Text>,
        emergency_contact_name -> Nullable<Text>,
        emergency_contact_phone -> Nullable<Text>,
        primary_physician_name -> Nullable<Text>,
        primary_physician_phone -> Nullable<Text>,
        insurance_provider -> Nullable<Text>,
        insurance_policy_number -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_medications (id) {
        id -> Text,
        student_id -> Text,
        medication_name -> Text,
        dosage -> Nullable<Text>,
        frequency -> Nullable<Text>,
        is_emergency_med -> Bool,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_missed_lessons (id) {
        id -> Text,
        student_id -> Text,
        lesson_progress_id -> Text,
        status -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        notified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    student_nics (id) {
        id -> Text,
        student_id -> Text,
        nic_number -> Text,
        issued_date -> Nullable<Date>,
        document_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_period_attendance (id) {
        id -> Text,
        student_id -> Text,
        class_id -> Text,
        timetable_id -> Text,
        date -> Date,
        status -> Text,
        minutes_late -> Nullable<Integer>,
        remarks -> Nullable<Text>,
        is_locked -> Bool,
        marked_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        suspicion_flag -> Nullable<Text>,
        detailed_status -> Nullable<Text>,
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
    student_status (student_id) {
        student_id -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    student_zscores (student_id, assessment_type, assessment_id, subject_id) {
        student_id -> Text,
        assessment_type -> Text,
        assessment_id -> Text,
        subject_id -> Text,
        zscore -> Float,
        zscore_formatted -> Text,
    }
}

diesel::table! {
    students (id) {
        id -> Text,
        admission_number -> Text,
        name_english -> Text,
        name_sinhala -> Nullable<Text>,
        name_tamil -> Nullable<Text>,
        dob -> Date,
        gender -> Text,
        profile_id -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subject_enrollments (student_id, subject_id, academic_year_id) {
        student_id -> Text,
        subject_id -> Text,
        academic_year_id -> Text,
        created_at -> Timestamp,
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
    substitution_plans (id) {
        id -> Text,
        subject_id -> Text,
        medium -> Text,
        plan_name -> Text,
        content_link -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    substitutions (id) {
        id -> Text,
        original_teacher_id -> Text,
        substitute_teacher_id -> Text,
        timetable_id -> Text,
        date -> Date,
        status -> Text,
        remarks -> Nullable<Text>,
        created_at -> Timestamp,
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
    teacher_period_attendance (id) {
        id -> Text,
        teacher_id -> Text,
        timetable_id -> Text,
        date -> Date,
        status -> Text,
        remarks -> Nullable<Text>,
        marked_by -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_substitution -> Bool,
        substitution_id -> Nullable<Text>,
    }
}

diesel::table! {
    teacher_reward_balances (teacher_id) {
        teacher_id -> Text,
        total_points -> Integer,
        updated_at -> Timestamp,
        lifetime_points -> Integer,
        last_updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    teacher_reward_details (reward_id) {
        reward_id -> Text,
        reason_type -> Text,
        reference_id -> Nullable<Text>,
        reward_type_id -> Nullable<Text>,
        awarded_by -> Nullable<Text>,
        status -> Text,
        effective_date -> Nullable<Date>,
        notes -> Nullable<Text>,
        reference_type -> Nullable<Text>,
        balance_after -> Nullable<Integer>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teacher_reward_history (id) {
        id -> Text,
        teacher_id -> Text,
        points -> Integer,
        created_at -> Timestamp,
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
        medium -> Text,
    }
}

diesel::table! {
    teacher_teaching_history (id) {
        id -> Text,
        staff_id -> Text,
        school_name -> Text,
        subject_id -> Nullable<Text>,
        grade_level_id -> Nullable<Text>,
        role_title -> Nullable<Text>,
        start_date -> Date,
        end_date -> Nullable<Date>,
        notes -> Nullable<Text>,
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
        subject_id -> Text,
        teacher_id -> Text,
        start_time -> Time,
        end_time -> Time,
        room -> Text,
        academic_year_id -> Text,
        grade_period_id -> Nullable<Text>,
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
    user_permissions (user_id, permission) {
        user_id -> Text,
        permission -> Text,
    }
}

diesel::table! {
    user_profiles (user_id, profile_id) {
        user_id -> Text,
        profile_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_security (user_id) {
        user_id -> Text,
        google_id -> Nullable<Text>,
        github_id -> Nullable<Text>,
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

diesel::table! {
    user_set_permissions (user_set_id, permission) {
        user_set_id -> Text,
        permission -> Text,
    }
}

diesel::table! {
    user_set_users (user_set_id, user_id) {
        user_set_id -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    user_sets (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user_status (user_id) {
        user_id -> Text,
        is_verified -> Bool,
        is_active -> Bool,
        disabled_at -> Nullable<Timestamp>,
        disabled_reason -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vendors (id) {
        id -> Text,
        name -> Text,
        contact_name -> Nullable<Text>,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        address -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    verification_tokens (id) {
        id -> Text,
        user_id -> Text,
        token_hash -> Text,
        purpose -> Text,
        issued_at -> Timestamp,
        expires_at -> Timestamp,
        consumed_at -> Nullable<Timestamp>,
        is_active -> Bool,
        metadata -> Nullable<Text>,
    }
}

diesel::table! {
    zscore_calculations (assessment_type, assessment_id, subject_id) {
        assessment_type -> Text,
        assessment_id -> Text,
        subject_id -> Text,
        mean -> Float,
        std_deviation -> Float,
        calculated_at -> Timestamp,
    }
}

diesel::joinable!(activities -> academic_years (academic_year_id));
diesel::joinable!(activities -> activity_types (activity_type_id));
diesel::joinable!(activities -> users (created_by));
diesel::joinable!(activity_attendance -> activities (activity_id));
diesel::joinable!(activity_participants -> activities (activity_id));
diesel::joinable!(activity_participants -> users (user_id));
diesel::joinable!(activity_participants_staff -> activities (activity_id));
diesel::joinable!(activity_participants_staff -> staff (staff_id));
diesel::joinable!(activity_participants_students -> activities (activity_id));
diesel::joinable!(activity_participants_students -> students (student_id));
diesel::joinable!(ai_processed_note_sections -> ai_processed_notes (note_id));
diesel::joinable!(ai_processed_notes -> lesson_materials (material_id));
diesel::joinable!(al_stream_grade_levels -> al_streams (stream_id));
diesel::joinable!(al_stream_grade_levels -> grade_levels (grade_level_id));
diesel::joinable!(al_stream_optional_groups -> al_streams (stream_id));
diesel::joinable!(al_stream_optional_subjects -> al_stream_optional_groups (group_id));
diesel::joinable!(al_stream_optional_subjects -> subjects (subject_id));
diesel::joinable!(al_stream_required_subjects -> al_streams (stream_id));
diesel::joinable!(al_stream_required_subjects -> subjects (subject_id));
diesel::joinable!(asset_allocations -> inventory_items (item_id));
diesel::joinable!(asset_allocations -> users (allocated_by));
diesel::joinable!(asset_allocations_staff -> asset_allocations (asset_allocation_id));
diesel::joinable!(asset_allocations_staff -> staff (staff_id));
diesel::joinable!(asset_allocations_students -> asset_allocations (asset_allocation_id));
diesel::joinable!(asset_allocations_students -> students (student_id));
diesel::joinable!(asset_maintenance_logs -> inventory_items (item_id));
diesel::joinable!(asset_maintenance_logs -> staff (performed_by));
diesel::joinable!(attendance_audit_log -> users (changed_by));
diesel::joinable!(attendance_discrepancies -> students (student_id));
diesel::joinable!(attendance_discrepancies -> users (resolved_by));
diesel::joinable!(attendance_excuses -> users (verified_by));
diesel::joinable!(audit_log -> users (user_id));
diesel::joinable!(auth_tokens -> users (user_id));
diesel::joinable!(behavior_incident_actions -> behavior_incidents (incident_id));
diesel::joinable!(behavior_incident_actions -> staff (assigned_to));
diesel::joinable!(behavior_incident_details -> behavior_incident_severity_levels (severity_id));
diesel::joinable!(behavior_incident_details -> behavior_incidents (incident_id));
diesel::joinable!(behavior_incident_details -> users (resolved_by));
diesel::joinable!(behavior_incident_evidence -> behavior_incidents (incident_id));
diesel::joinable!(behavior_incident_evidence -> users (uploaded_by));
diesel::joinable!(behavior_incident_followups -> behavior_incidents (incident_id));
diesel::joinable!(behavior_incident_followups -> users (recorded_by));
diesel::joinable!(behavior_incident_participants -> behavior_incidents (incident_id));
diesel::joinable!(behavior_incidents -> behavior_incident_types (incident_type_id));
diesel::joinable!(behavior_incidents -> students (student_id));
diesel::joinable!(behavior_incidents -> users (reported_by_user_id));
diesel::joinable!(budgets -> academic_years (academic_year_id));
diesel::joinable!(budgets -> budget_categories (category_id));
diesel::joinable!(class_subject_teachers -> academic_years (academic_year_id));
diesel::joinable!(class_subject_teachers -> classes (class_id));
diesel::joinable!(class_subject_teachers -> staff (teacher_id));
diesel::joinable!(class_subject_teachers -> subjects (subject_id));
diesel::joinable!(classes -> academic_years (academic_year_id));
diesel::joinable!(classes -> grade_levels (grade_id));
diesel::joinable!(classes -> school_rooms (room_id));
diesel::joinable!(classes -> staff (class_teacher_id));
diesel::joinable!(club_activities -> clubs (club_id));
diesel::joinable!(club_members -> clubs (club_id));
diesel::joinable!(club_members -> students (student_id));
diesel::joinable!(clubs -> staff (teacher_in_charge_id));
diesel::joinable!(competition_participants -> competitions (competition_id));
diesel::joinable!(competition_participants -> students (student_id));
diesel::joinable!(conversation_participants -> conversations (conversation_id));
diesel::joinable!(conversation_participants -> users (user_id));
diesel::joinable!(cultural_event_participants -> cultural_events (event_id));
diesel::joinable!(cultural_event_participants -> students (student_id));
diesel::joinable!(curriculum_standards -> al_streams (stream_id));
diesel::joinable!(curriculum_standards -> grade_levels (grade_level_id));
diesel::joinable!(curriculum_standards -> subjects (subject_id));
diesel::joinable!(curriculum_topics -> curriculum_standards (curriculum_standard_id));
diesel::joinable!(detention_balances -> students (student_id));
diesel::joinable!(emergency_roll_call_entries -> emergency_roll_calls (roll_call_id));
diesel::joinable!(emergency_roll_call_entries -> users (user_id));
diesel::joinable!(emergency_roll_calls -> users (initiated_by));
diesel::joinable!(exam_structure_subjects -> exam_structures (structure_id));
diesel::joinable!(exam_structure_subjects -> subjects (subject_id));
diesel::joinable!(exit_passes -> exit_passes_bulk (bulk_pass_id));
diesel::joinable!(exit_passes -> students (student_id));
diesel::joinable!(exit_passes -> users (approved_by));
diesel::joinable!(exit_passes_bulk -> users (issued_by));
diesel::joinable!(expense_transactions -> expense_categories (category_id));
diesel::joinable!(expense_transactions -> staff (approved_by));
diesel::joinable!(fee_invoice_items -> fee_invoices (invoice_id));
diesel::joinable!(fee_invoice_items -> fee_structure_items (fee_structure_item_id));
diesel::joinable!(fee_invoices -> academic_years (academic_year_id));
diesel::joinable!(fee_invoices -> students (student_id));
diesel::joinable!(fee_invoices -> terms (term_id));
diesel::joinable!(fee_payment_allocations -> fee_invoices (invoice_id));
diesel::joinable!(fee_payment_allocations -> fee_payments (payment_id));
diesel::joinable!(fee_payment_details -> fee_payments (payment_id));
diesel::joinable!(fee_payment_details -> users (recorded_by));
diesel::joinable!(fee_payments -> staff (collected_by));
diesel::joinable!(fee_payments -> student_fees (student_fee_id));
diesel::joinable!(fee_structure_items -> fee_structures (fee_structure_id));
diesel::joinable!(fee_structure_pricing -> fee_structures (fee_structure_id));
diesel::joinable!(fee_structure_schedule -> fee_structures (fee_structure_id));
diesel::joinable!(fee_structures -> academic_years (academic_year_id));
diesel::joinable!(fee_structures -> fee_categories (category_id));
diesel::joinable!(fee_structures -> grade_levels (grade_id));
diesel::joinable!(exam_subjects -> exams (exam_id));
diesel::joinable!(exam_subjects -> subjects (subject_id));
diesel::joinable!(government_exam_subjects -> government_exams (government_exam_id));
diesel::joinable!(government_exam_subjects -> subjects (subject_id));
diesel::joinable!(government_exams -> exam_structures (exam_structure_id));
diesel::joinable!(grade_periods -> grade_levels (grade_id));
diesel::joinable!(grade_subjects -> grade_levels (grade_id));
diesel::joinable!(grade_subjects -> subjects (subject_id));
diesel::joinable!(grading_criteria -> grading_schemes (scheme_id));
diesel::joinable!(grading_schemes -> grade_levels (grade_level_id));
diesel::joinable!(income_transactions -> income_sources (source_id));
diesel::joinable!(income_transactions -> staff (received_by));
diesel::joinable!(inventory_item_details -> inventory_items (item_id));
diesel::joinable!(inventory_items -> asset_categories (category_id));
diesel::joinable!(inventory_transactions -> inventory_items (item_id));
diesel::joinable!(ledger_entries -> chart_of_accounts (account_id));
diesel::joinable!(ledger_entries -> ledger_transactions (transaction_id));
diesel::joinable!(lesson_materials -> lesson_progress (lesson_progress_id));
diesel::joinable!(lesson_materials -> staff (uploader_id));
diesel::joinable!(lesson_progress -> classes (class_id));
diesel::joinable!(lesson_progress -> curriculum_topics (curriculum_topic_id));
diesel::joinable!(lesson_progress -> subjects (subject_id));
diesel::joinable!(lesson_progress -> timetable (timetable_id));
diesel::joinable!(lesson_progress_attachments -> lesson_progress (lesson_progress_id));
diesel::joinable!(lesson_progress_periods -> lesson_progress (lesson_progress_id));
diesel::joinable!(lesson_progress_periods -> timetable (timetable_id));
diesel::joinable!(lesson_reviews -> lesson_progress (lesson_progress_id));
diesel::joinable!(library_books -> library_categories (category_id));
diesel::joinable!(library_issues -> library_books (book_id));
diesel::joinable!(library_issues -> staff (staff_id));
diesel::joinable!(library_issues -> students (student_id));
diesel::joinable!(maintenance_requests -> inventory_items (item_id));
diesel::joinable!(maintenance_requests -> staff (assigned_to));
diesel::joinable!(maintenance_requests -> users (reported_by));
diesel::joinable!(marking_scheme_parts -> marking_schemes (scheme_id));
diesel::joinable!(marking_schemes -> al_streams (stream_id));
diesel::joinable!(marking_schemes -> curriculum_standards (curriculum_standard_id));
diesel::joinable!(marking_schemes -> grade_levels (grade_level_id));
diesel::joinable!(marking_schemes -> subjects (subject_id));
diesel::joinable!(messages -> conversations (conversation_id));
diesel::joinable!(messages -> users (sender_user_id));
diesel::joinable!(petty_cash_transactions -> staff (handled_by));
diesel::joinable!(practical_lesson_appeals -> lesson_progress (lesson_progress_id));
diesel::joinable!(practical_lesson_appeals -> staff (reviewed_by));
diesel::joinable!(pre_approved_absences -> students (student_id));
diesel::joinable!(pre_approved_absences -> users (approved_by));
diesel::joinable!(profile_contacts -> profiles (profile_id));
diesel::joinable!(profile_media -> profiles (profile_id));
diesel::joinable!(purchase_order_items -> purchase_orders (purchase_order_id));
diesel::joinable!(purchase_orders -> users (created_by));
diesel::joinable!(purchase_orders -> vendors (vendor_id));
diesel::joinable!(report_card_marks -> marking_schemes (marking_scheme_id));
diesel::joinable!(report_card_marks -> report_cards (report_card_id));
diesel::joinable!(report_card_marks -> subjects (subject_id));
diesel::joinable!(report_cards -> academic_years (academic_year_id));
diesel::joinable!(report_cards -> classes (class_id));
diesel::joinable!(report_cards -> grading_schemes (grading_scheme_id));
diesel::joinable!(report_cards -> students (student_id));
diesel::joinable!(report_cards -> terms (term_id));
diesel::joinable!(report_cards -> users (generated_by));
diesel::joinable!(resource_assets -> inventory_items (inventory_item_id));
diesel::joinable!(resource_assets -> resources (resource_id));
diesel::joinable!(resource_bookings -> resources (resource_id));
diesel::joinable!(resource_bookings -> users (booked_by_user_id));
diesel::joinable!(resource_details -> resources (resource_id));
diesel::joinable!(reward_adjustments -> staff (teacher_id));
diesel::joinable!(reward_adjustments -> users (approved_by));
diesel::joinable!(role_set_roles -> role_sets (role_set_id));
diesel::joinable!(salary_payments -> staff (staff_id));
diesel::joinable!(school_test_subjects -> school_tests (school_test_id));
diesel::joinable!(school_test_subjects -> subjects (subject_id));
diesel::joinable!(school_tests -> academic_years (academic_year_id));
diesel::joinable!(school_tests -> exam_structures (exam_structure_id));
diesel::joinable!(school_tests -> terms (term_id));
diesel::joinable!(school_tests -> users (created_by));
diesel::joinable!(sessions -> auth_tokens (auth_token_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(sessions -> verification_tokens (verification_token_id));
diesel::joinable!(sport_event_participants -> sport_events (event_id));
diesel::joinable!(sport_event_participants -> sport_teams (team_id));
diesel::joinable!(sport_event_participants -> students (student_id));
diesel::joinable!(sport_events -> sports (sport_id));
diesel::joinable!(sport_team_members -> sport_teams (team_id));
diesel::joinable!(sport_team_members -> students (student_id));
diesel::joinable!(sport_teams -> sports (sport_id));
diesel::joinable!(sport_teams -> staff (coach_id));
diesel::joinable!(staff -> profiles (profile_id));
diesel::joinable!(staff_attendance -> staff (staff_id));
diesel::joinable!(staff_attendance -> staff_events (event_id));
diesel::joinable!(staff_contacts -> staff (staff_id));
diesel::joinable!(staff_contracts -> staff (staff_id));
diesel::joinable!(staff_cvs -> staff (staff_id));
diesel::joinable!(staff_documents -> staff (staff_id));
diesel::joinable!(staff_employment_history -> staff (staff_id));
diesel::joinable!(staff_employment_status -> staff (staff_id));
diesel::joinable!(staff_event_participants -> staff (staff_id));
diesel::joinable!(staff_event_participants -> staff_events (event_id));
diesel::joinable!(staff_identity -> staff (staff_id));
diesel::joinable!(staff_leave_balances -> staff (staff_id));
diesel::joinable!(staff_leave_balances -> staff_leave_types (leave_type_id));
diesel::joinable!(staff_leave_requests -> staff (staff_id));
diesel::joinable!(staff_leave_requests -> staff_leave_types (leave_type_id));
diesel::joinable!(staff_leave_requests -> users (approved_by));
diesel::joinable!(staff_leaves -> staff (staff_id));
diesel::joinable!(staff_media -> staff (staff_id));
diesel::joinable!(staff_notes -> staff (staff_id));
diesel::joinable!(staff_notes -> users (created_by));
diesel::joinable!(staff_overtime -> staff (staff_id));
diesel::joinable!(staff_overtime -> users (approved_by));
diesel::joinable!(staff_qualifications -> staff (staff_id));
diesel::joinable!(staff_reward_snapshots -> staff (staff_id));
diesel::joinable!(staff_salaries -> salary_components (component_id));
diesel::joinable!(staff_salaries -> staff (staff_id));
diesel::joinable!(staff_skills -> staff (staff_id));
diesel::joinable!(staff_subject_expertise -> staff (staff_id));
diesel::joinable!(staff_subject_expertise -> subjects (subject_id));
diesel::joinable!(staff_subjects -> staff (staff_id));
diesel::joinable!(staff_subjects -> subjects (subject_id));
diesel::joinable!(student_achievements -> students (student_id));
diesel::joinable!(student_allergies -> students (student_id));
diesel::joinable!(student_attendance -> classes (class_id));
diesel::joinable!(student_attendance -> students (student_id));
diesel::joinable!(student_attendance -> users (marked_by));
diesel::joinable!(student_birth_certificates -> students (student_id));
diesel::joinable!(student_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(student_class_assignments -> classes (class_id));
diesel::joinable!(student_class_assignments -> grade_levels (grade_id));
diesel::joinable!(student_class_assignments -> students (student_id));
diesel::joinable!(student_contacts -> students (student_id));
diesel::joinable!(student_demographics -> students (student_id));
diesel::joinable!(student_emergency_contacts -> students (student_id));
diesel::joinable!(student_fees -> fee_structures (fee_structure_id));
diesel::joinable!(student_fees -> students (student_id));
diesel::joinable!(student_guardians -> students (student_id));
diesel::joinable!(student_guardians -> users (user_id));
diesel::joinable!(student_mark_entries -> marking_scheme_parts (marking_scheme_part_id));
diesel::joinable!(student_mark_entries -> student_marks (student_mark_id));
diesel::joinable!(student_mark_entries_history -> marking_scheme_parts (marking_scheme_part_id));
diesel::joinable!(student_mark_entries_history -> student_marks_history (student_marks_history_id));
diesel::joinable!(student_marks -> marking_schemes (marking_scheme_id));
diesel::joinable!(student_marks -> students (student_id));
diesel::joinable!(student_marks -> subjects (subject_id));
diesel::joinable!(student_marks_history -> marking_schemes (marking_scheme_id));
diesel::joinable!(student_marks_history -> students (student_id));
diesel::joinable!(student_marks_history -> subjects (subject_id));
diesel::joinable!(student_media -> students (student_id));
diesel::joinable!(student_medical_conditions -> students (student_id));
diesel::joinable!(student_medical_info -> students (student_id));
diesel::joinable!(student_medications -> students (student_id));
diesel::joinable!(student_missed_lessons -> lesson_progress (lesson_progress_id));
diesel::joinable!(student_missed_lessons -> students (student_id));
diesel::joinable!(student_nics -> students (student_id));
diesel::joinable!(student_period_attendance -> classes (class_id));
diesel::joinable!(student_period_attendance -> students (student_id));
diesel::joinable!(student_period_attendance -> timetable (timetable_id));
diesel::joinable!(student_period_attendance -> users (marked_by));
diesel::joinable!(student_previous_schools -> students (student_id));
diesel::joinable!(student_status -> students (student_id));
diesel::joinable!(student_zscores -> students (student_id));
diesel::joinable!(student_zscores -> subjects (subject_id));
diesel::joinable!(students -> profiles (profile_id));
diesel::joinable!(subject_enrollments -> academic_years (academic_year_id));
diesel::joinable!(subject_enrollments -> students (student_id));
diesel::joinable!(subject_enrollments -> subjects (subject_id));
diesel::joinable!(substitution_plans -> subjects (subject_id));
diesel::joinable!(substitutions -> timetable (timetable_id));
diesel::joinable!(teacher_class_assignments -> academic_years (academic_year_id));
diesel::joinable!(teacher_class_assignments -> classes (class_id));
diesel::joinable!(teacher_class_assignments -> staff (teacher_id));
diesel::joinable!(teacher_period_attendance -> staff (teacher_id));
diesel::joinable!(teacher_period_attendance -> substitutions (substitution_id));
diesel::joinable!(teacher_period_attendance -> timetable (timetable_id));
diesel::joinable!(teacher_period_attendance -> users (marked_by));
diesel::joinable!(teacher_reward_balances -> staff (teacher_id));
diesel::joinable!(teacher_reward_details -> reward_types (reward_type_id));
diesel::joinable!(teacher_reward_details -> teacher_reward_history (reward_id));
diesel::joinable!(teacher_reward_details -> users (awarded_by));
diesel::joinable!(teacher_reward_history -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> academic_years (academic_year_id));
diesel::joinable!(teacher_subject_assignments -> staff (teacher_id));
diesel::joinable!(teacher_subject_assignments -> subjects (subject_id));
diesel::joinable!(teacher_teaching_history -> grade_levels (grade_level_id));
diesel::joinable!(teacher_teaching_history -> staff (staff_id));
diesel::joinable!(teacher_teaching_history -> subjects (subject_id));
diesel::joinable!(terms -> academic_years (academic_year_id));
diesel::joinable!(timetable -> academic_years (academic_year_id));
diesel::joinable!(timetable -> classes (class_id));
diesel::joinable!(timetable -> grade_periods (grade_period_id));
diesel::joinable!(timetable -> staff (teacher_id));
diesel::joinable!(timetable -> subjects (subject_id));
diesel::joinable!(uniform_issues -> staff (issued_by));
diesel::joinable!(uniform_issues -> students (student_id));
diesel::joinable!(uniform_issues -> uniform_items (uniform_item_id));
diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_profiles -> profiles (profile_id));
diesel::joinable!(user_profiles -> users (user_id));
diesel::joinable!(user_security -> users (user_id));
diesel::joinable!(user_set_permissions -> user_sets (user_set_id));
diesel::joinable!(user_set_users -> user_sets (user_set_id));
diesel::joinable!(user_set_users -> users (user_id));
diesel::joinable!(user_status -> users (user_id));
diesel::joinable!(verification_tokens -> users (user_id));
diesel::joinable!(zscore_calculations -> subjects (subject_id));

diesel::allow_tables_to_appear_in_same_query!(
    academic_years,
    activities,
    activity_attendance,
    activity_participants,
    activity_participants_staff,
    activity_participants_students,
    activity_types,
    ai_processed_note_sections,
    ai_processed_notes,
    al_stream_grade_levels,
    al_stream_optional_groups,
    al_stream_optional_subjects,
    al_stream_required_subjects,
    al_streams,
    asset_allocations,
    asset_allocations_staff,
    asset_allocations_students,
    asset_categories,
    asset_maintenance_logs,
    attendance_audit_log,
    attendance_discrepancies,
    attendance_excuses,
    attendance_policies,
    audit_log,
    auth_tokens,
    behavior_incident_actions,
    behavior_incident_details,
    behavior_incident_evidence,
    behavior_incident_followups,
    behavior_incident_participants,
    behavior_incident_severity_levels,
    behavior_incident_types,
    behavior_incidents,
    budget_categories,
    budgets,
    chart_of_accounts,
    class_subject_teachers,
    classes,
    club_activities,
    club_members,
    clubs,
    competition_participants,
    competitions,
    conversation_participants,
    conversations,
    cultural_event_participants,
    cultural_events,
    curriculum_standards,
    curriculum_topics,
    detention_balances,
    emergency_roll_call_entries,
    emergency_roll_calls,
    exam_subjects,
    exam_structure_subjects,
    exam_structures,
    exams,
    exit_passes,
    exit_passes_bulk,
    expense_categories,
    expense_transactions,
    fee_categories,
    fee_invoice_items,
    fee_invoices,
    fee_payment_allocations,
    fee_payment_details,
    fee_payments,
    fee_structure_items,
    fee_structure_pricing,
    fee_structure_schedule,
    fee_structures,
    general_ledger,
    government_exam_subjects,
    government_exams,
    grade_levels,
    grade_periods,
    grade_subjects,
    grading_criteria,
    grading_schemes,
    income_sources,
    income_transactions,
    inventory_item_details,
    inventory_items,
    inventory_transactions,
    ledger_entries,
    ledger_transactions,
    lesson_materials,
    lesson_progress,
    lesson_progress_attachments,
    lesson_progress_periods,
    lesson_reviews,
    library_books,
    library_categories,
    library_issues,
    library_settings,
    maintenance_requests,
    marking_scheme_parts,
    marking_schemes,
    messages,
    petty_cash_transactions,
    practical_lesson_appeals,
    pre_approved_absences,
    profile_contacts,
    profile_media,
    profiles,
    purchase_order_items,
    purchase_orders,
    report_card_marks,
    report_cards,
    resource_assets,
    resource_bookings,
    resource_details,
    resources,
    reward_adjustments,
    reward_types,
    role_permissions,
    role_set_roles,
    role_sets,
    salary_components,
    salary_payments,
    school_calendar,
    school_rooms,
    school_settings,
    school_test_subjects,
    school_tests,
    seeds,
    sessions,
    sport_event_participants,
    sport_events,
    sport_team_members,
    sport_teams,
    sports,
    staff,
    staff_attendance,
    staff_contacts,
    staff_contracts,
    staff_cvs,
    staff_departments,
    staff_documents,
    staff_employment_history,
    staff_employment_status,
    staff_event_participants,
    staff_events,
    staff_identity,
    staff_leave_balances,
    staff_leave_requests,
    staff_leave_types,
    staff_leaves,
    staff_media,
    staff_notes,
    staff_overtime,
    staff_qualifications,
    staff_reward_snapshots,
    staff_salaries,
    staff_skills,
    staff_subject_expertise,
    staff_subjects,
    student_achievements,
    student_allergies,
    student_attendance,
    student_birth_certificates,
    student_class_assignments,
    student_class_assignments_history,
    student_contacts,
    student_demographics,
    student_emergency_contacts,
    student_fees,
    student_guardians,
    student_mark_entries,
    student_mark_entries_history,
    student_marks,
    student_marks_history,
    student_media,
    student_medical_conditions,
    student_medical_info,
    student_medications,
    student_missed_lessons,
    student_nics,
    student_period_attendance,
    student_previous_schools,
    student_status,
    student_zscores,
    students,
    subject_enrollments,
    subjects,
    substitution_plans,
    substitutions,
    teacher_class_assignments,
    teacher_period_attendance,
    teacher_reward_balances,
    teacher_reward_details,
    teacher_reward_history,
    teacher_subject_assignments,
    teacher_teaching_history,
    terms,
    timetable,
    uniform_issues,
    uniform_items,
    user_permissions,
    user_profiles,
    user_security,
    user_set_permissions,
    user_set_users,
    user_sets,
    user_status,
    users,
    vendors,
    verification_tokens,
    zscore_calculations,
    files,
);

diesel::table! {
    files (id) {
        id -> Text,
        file_name -> Text,
        file_path -> Text,
        mime_type -> Text,
        file_size -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
