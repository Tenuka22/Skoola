use apistos::ApiComponent;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent, PartialEq, Eq, Hash)]
        pub struct $name(pub String);
    };
}

id_type!(UserId);
id_type!(ProfileId);
id_type!(StaffId);
id_type!(StudentId);
id_type!(ClassId);
id_type!(SubjectId);
id_type!(AcademicYearId);
id_type!(GradeLevelId);
id_type!(TermId);
id_type!(TimetableId);
id_type!(SchoolRoomId);
id_type!(AlStreamId);
id_type!(CurriculumStandardId);
id_type!(CurriculumTopicId);
id_type!(MarkingSchemeId);
id_type!(ExamStructureId);
id_type!(ExamStructureSubjectId);
id_type!(GovernmentExamId);
id_type!(GovernmentExamSubjectId);
id_type!(SchoolTestId);
id_type!(SchoolTestSubjectId);
id_type!(ReportCardId);
id_type!(ReportCardMarkId);
id_type!(GradingSchemeId);
id_type!(FeeStructureId);
id_type!(InventoryItemId);
id_type!(MarkingSchemePartId);
id_type!(StudentMarkId);
id_type!(AssessmentId);

#[derive(Debug, Clone, Copy)]
pub struct IdPrefix(pub &'static str);

impl IdPrefix {
    pub const USER: Self = Self("usr");
    pub const PROFILE: Self = Self("pro");
    pub const STAFF: Self = Self("stf");
    pub const STUDENT: Self = Self("stu");
    pub const GUARDIAN: Self = Self("gdn");
    pub const AUTH_TOKEN: Self = Self("atk");
    pub const VERIFICATION_TOKEN: Self = Self("vtk");
    pub const SESSION: Self = Self("ses");
    pub const MESSAGE: Self = Self("msg");
    pub const CONVERSATION: Self = Self("con");
    pub const RESOURCE: Self = Self("res");
    pub const BOOKING: Self = Self("bkd");
    pub const STUDENT_MARK: Self = Self("smk");
    pub const CLASS_ASSIGNMENT: Self = Self("cas");
    pub const ROLL_CALL: Self = Self("rcl");
    pub const ATTENDANCE: Self = Self("att");
    pub const REWARD: Self = Self("rwd");
    pub const ACTIVITY: Self = Self("act");
    pub const AUDIT: Self = Self("aud");
    pub const STUDENT_CLASS_ASSIGNMENT: Self = Self("sca");
    pub const ATTENDANCE_POLICY: Self = Self("apl");
    pub const PROPERTY_ALLOCATION: Self = Self("pal");
    pub const PROPERTY: Self = Self("prp");
    pub const FINANCIAL_ALLOCATION: Self = Self("fal");
    pub const FINANCIAL: Self = Self("fin");
    pub const FEE: Self = Self("fee");
    pub const FEE_STRUCTURE: Self = Self("fst");
    pub const CO_CURRICULAR_ALLOCATION: Self = Self("cca");
    pub const CO_CURRICULAR: Self = Self("coc");
    pub const GOVERNMENT_EXAM: Self = Self("gex");
    pub const SCHOOL_TEST: Self = Self("stt");
    pub const REPORT_CARD: Self = Self("rcd");
    pub const MARKING_SCHEME: Self = Self("mks");
    pub const MARKING_SCHEME_PART: Self = Self("mkp");
    pub const ASSESSMENT: Self = Self("asm");
    pub const GRADING_SCHEME: Self = Self("gsc");
    pub const GRADING_CRITERION: Self = Self("gcr");
    pub const EXAM_TYPE: Self = Self("ext");
    pub const EXAM_STRUCTURE: Self = Self("exs");
    pub const EXAM: Self = Self("exm");
    pub const LEDGER: Self = Self("ldg");
    pub const ACCOUNT: Self = Self("acc");
    pub const BEHAVIOR: Self = Self("beh");
    pub const SYLLABUS: Self = Self("syl");
    pub const LESSON_PROGRESS: Self = Self("lsp");
    pub const CURRICULUM: Self = Self("cur");
    pub const APPEAL: Self = Self("apl");
    pub const ATTACHMENT: Self = Self("att");
    pub const REVIEW: Self = Self("rev");
    pub const MATERIAL: Self = Self("mat");
    pub const TERM: Self = Self("trm");
    pub const SUBSTITUTION_PLAN: Self = Self("spl");
    pub const SUBJECT: Self = Self("sbj");
    pub const AL_STREAM: Self = Self("als");
    pub const TIMETABLE: Self = Self("ttb");
    pub const GRADE_LEVEL: Self = Self("grl");
    pub const GRADE_PERIOD: Self = Self("grp");
    pub const CLASS: Self = Self("cls");
    pub const ACADEMIC_YEAR: Self = Self("acy");
    pub const TEACHER_ASSIGNMENT: Self = Self("tas");
    pub const ROLE_SET: Self = Self("rst");
    pub const PERMISSION_SET: Self = Self("pst");
    pub const LEAVE: Self = Self("lev");
    pub const SEED: Self = Self("sed");
}

#[derive(diesel::QueryableByName)]
struct NextIdRow {
    #[diesel(sql_type = BigInt)]
    next_id: i64,
}

fn ensure_id_sequence_table(conn: &mut SqliteConnection) -> QueryResult<()> {
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS id_sequences (
            prefix TEXT PRIMARY KEY,
            next_id INTEGER NOT NULL
        )",
    )
    .execute(conn)?;
    Ok(())
}

pub fn generate_prefixed_id(
    conn: &mut SqliteConnection,
    prefix: IdPrefix,
) -> QueryResult<String> {
    ensure_id_sequence_table(conn)?;

    // Atomically create/increment the next numeric value for a prefix.
    diesel::sql_query(
        "INSERT INTO id_sequences(prefix, next_id)
         VALUES (?1, 1)
         ON CONFLICT(prefix) DO UPDATE SET next_id = next_id + 1",
    )
    .bind::<Text, _>(prefix.0)
    .execute(conn)?;

    let row = diesel::sql_query("SELECT next_id FROM id_sequences WHERE prefix = ?1")
        .bind::<Text, _>(prefix.0)
        .get_result::<NextIdRow>(conn)?;

    Ok(format!("{}_{:06}", prefix.0, row.next_id))
}
