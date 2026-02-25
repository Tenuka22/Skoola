pub mod exam;
pub mod exam_type;
pub mod exam_subject;
pub mod grading_scheme;
pub mod grading_criterion;
pub mod student_marks;
pub mod report_card;
pub mod zscore;
pub mod special_exam;
pub mod report_card_mark; // New
pub mod al_exam; // New
pub mod ol_exam; // New
pub mod scholarship_exam; // New

pub use exam::*;
pub use exam_type::*;
pub use exam_subject::*;
pub use grading_scheme::*;
pub use grading_criterion::*;
pub use student_marks::*;
pub use report_card::*;
pub use zscore::*;
pub use special_exam::ExamRegistrationRequest;
pub use al_exam::*;
pub use ol_exam::*;
pub use scholarship_exam::*;
