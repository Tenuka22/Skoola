pub mod academic;
pub mod auth;
pub mod behavior_management;
pub mod curriculum_management;
pub mod exams;
pub mod finance;
pub mod messaging;
pub mod resource_management;
pub mod resources;
pub mod staff;
pub mod student;
pub mod system;

// Unambiguous re-exports can be added here if needed,
// but it's often clearer to use the full path.
pub use academic::academic_year;
pub use academic::class;
pub use academic::class_subject_teacher;
pub use academic::grade_level;
pub use academic::structure;
pub use academic::subject;
pub use academic::terms;
pub use academic::timetable;

pub use auth::NewProfile;
pub use auth::NewUserProfile;
pub use auth::Profile;
pub use auth::UserProfile;
pub use auth::permission as auth_permission;
pub use auth::profile as auth_profile;
pub use auth::role as auth_role;
pub use auth::session as auth_session;
pub use auth::user as auth_user;

pub use exams::exam;
pub use exams::exam_subject;
pub use exams::exam_type;
pub use exams::grading_criterion;
pub use exams::grading_scheme;
pub use exams::report_card;
pub use exams::special_exam;
pub use exams::student_marks;
pub use exams::zscore;

pub use finance::account; // Added
pub use finance::budget;
pub use finance::fees;
pub use finance::ledger;
pub use finance::salary;
pub use finance::transaction; // Added

pub use resources::co_curricular;
pub use resources::inventory;
pub use resources::library;

pub use staff::assignment as staff_assignment;
pub use staff::attendance as staff_attendance;
pub use staff::department as staff_department;
pub use staff::history as staff_history;
pub use staff::leave as staff_leave;
pub use staff::qualification as staff_qualification;
pub use staff::staff as staff_member;

pub use student::attendance as student_attendance;
pub use student::contact as student_contact;
pub use student::enrollment as student_enrollment;
pub use student::guardian as student_guardian;
pub use student::history as student_history;
pub use student::medical as student_medical;
pub use student::student as student_member;

pub use system::activity;
pub use system::calendar;
pub use system::seed;
pub use system::setting;

// Add this line to re-export resource_management models
pub use resource_management::*;

use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub message: String,
}
