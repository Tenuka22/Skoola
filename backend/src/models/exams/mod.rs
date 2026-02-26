pub mod al_exam; // New
pub mod exam;
pub mod exam_subject;
pub mod exam_type;
pub mod grading_criterion;
pub mod grading_scheme;
pub mod ol_exam; // New
pub mod report_card;
pub mod report_card_mark; // New
pub mod scholarship_exam;
pub mod special_exam;
pub mod student_marks;
pub mod zscore; // New

pub use al_exam::*;
pub use exam::*;
pub use exam_subject::*;
pub use exam_type::*;
pub use grading_criterion::*;
pub use grading_scheme::*;
pub use ol_exam::*;
pub use report_card::*;
pub use scholarship_exam::*;
pub use special_exam::ExamRegistrationRequest;
pub use student_marks::*;
pub use zscore::*;
