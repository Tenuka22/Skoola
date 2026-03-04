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

pub use academic::*;
pub use auth::*;
pub use behavior_management::*;
pub use curriculum_management::*;
pub use exams::*;
pub use finance::*;
pub use messaging::*;
pub use resource_management::*;
pub use resources::*;
pub use staff::*;
pub use student::*;
pub use system::*;

use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub message: String,
}
