pub mod academic_year;
pub mod academic_structure;
pub mod auth;
pub mod class_subject_teacher; // Added
pub mod class;
pub mod co_curricular;
pub mod exam_subjects;
pub mod exam_types;
pub mod exams;
pub mod fees;
pub mod financial;
pub mod grade_level;
pub mod grading_criteria;
pub mod grading_schemes;
pub mod library;
pub mod profile;
pub mod property; // Added
pub mod report_cards;
pub mod roles;
pub mod special_exams;
pub mod staff_attendance;
pub mod staff_leaves;
pub mod staff_roles;
pub mod staff;
pub mod student_attendance;
pub mod student_class_assignment;
pub mod student_emergency_contact;
pub mod student_guardian;
pub mod student_marks;
pub mod student_medical_info;
pub mod student_previous_school;
pub mod student;
pub mod subject;
pub mod teacher_assignments;
pub mod terms;
pub mod timetable;
pub mod zscore;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub message: String,
}