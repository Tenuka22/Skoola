pub mod academic;
pub mod auth;
pub mod behavior_management;
pub mod curriculum_management;
pub mod exams;
pub mod finance;
pub mod ids;
pub mod messaging;
pub mod resources;
pub mod staff;
pub mod student;
pub mod system;

pub use academic::*;
pub use auth::*;
pub use behavior_management::*;
pub use curriculum_management::*;
pub use exams::*;
pub use finance::*;
pub use ids::*;
pub use messaging::*;
pub use resources::*;
pub use staff::{
    Staff, StaffQuery, StaffResponse, attendance as StaffAttendance, history as StaffHistory,
    LessonProgress as StaffLessonProgress,
};
pub use student::{
    Student, StudentQuery, StudentResponse, attendance as StudentAttendance,
    history as StudentHistory, DetentionBalance as StudentDetentionBalance,
};
pub use system::{
    FileModel, FileQuery, FileResponse,
    SchoolSettingResponse, UpdateSchoolSettingRequest,
};

use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub message: String,
}
