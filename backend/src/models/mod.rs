pub mod academic;
pub mod auth;
pub mod exams;
pub mod finance;
pub mod resources;
pub mod staff;
pub mod student;
pub mod system;

// Unambiguous re-exports can be added here if needed, 
// but it's often clearer to use the full path.
pub use academic::academic_year;
pub use academic::terms;
pub use academic::grade_level;
pub use academic::class;
pub use academic::subject;
pub use academic::timetable;
pub use academic::class_subject_teacher;
pub use academic::structure;

pub use auth::user as auth_user;
pub use auth::session as auth_session;
pub use auth::permission as auth_permission;
pub use auth::profile as auth_profile;
pub use auth::role as auth_role;
pub use auth::Profile;
pub use auth::NewProfile;
pub use auth::UserProfile;
pub use auth::NewUserProfile;

pub use exams::exam;
pub use exams::exam_type;
pub use exams::exam_subject;
pub use exams::grading_scheme;
pub use exams::grading_criterion;
pub use exams::student_marks;
pub use exams::report_card;
pub use exams::zscore;
pub use exams::special_exam;

pub use finance::fees;
pub use finance::budget;
pub use finance::transaction;
pub use finance::salary;
pub use finance::account; // Added
pub use finance::ledger; // Added

pub use resources::library;
pub use resources::inventory;
pub use resources::co_curricular;

pub use staff::staff as staff_member;
pub use staff::attendance as staff_attendance;
pub use staff::leave as staff_leave;
pub use staff::assignment as staff_assignment;
pub use staff::history as staff_history;
pub use staff::qualification as staff_qualification;
pub use staff::department as staff_department;

pub use student::student as student_member;
pub use student::attendance as student_attendance;
pub use student::enrollment as student_enrollment;
pub use student::guardian as student_guardian;
pub use student::medical as student_medical;
pub use student::contact as student_contact;
pub use student::history as student_history;

pub use system::activity;
pub use system::setting;
pub use system::calendar;
pub use system::seed;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub message: String,
}
