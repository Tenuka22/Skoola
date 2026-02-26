use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

macro_rules! diesel_text_enum {
    ($(#[$outer:meta])* pub enum $name:ident {
        $($variant:ident),+ $(,)?
    }) => {
        $(#[$outer])*
        #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq, Eq, Hash)]
        #[diesel(sql_type = Text)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                }
            }

            pub fn from_str_case_sensitive(s: &str) -> std::result::Result<Self, &'static str> {
                match s {
                    $(stringify!($variant) => Ok(Self::$variant),)+
                    _ => Err("Invalid enum variant"),
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::from_str_case_sensitive(s)
            }
        }

        impl ToSql<Text, Sqlite> for $name {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
                out.set_value(self.as_str());
                Ok(IsNull::No)
            }
        }

        impl FromSql<Text, Sqlite> for $name {
            fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
                let s = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
                Self::from_str_case_sensitive(&s).map_err(|_| "Unrecognized enum variant".into())
            }
        }
    };
}

// Basic roles and employment metadata

diesel_text_enum! {
    pub enum RoleEnum {
        Admin,
        Teacher,
        Student,
        Guest,
        Parent,
        FullAdmin,
        Principal,
        VicePrincipal,
        Accountant,
        Librarian,
    }
}

diesel_text_enum! {
    pub enum EmploymentStatus {
        Permanent,
        Contract,
        Temporary,
    }
}

diesel_text_enum! {
    pub enum StaffType {
        Teaching,
        NonTeaching,
        Administrative,
    }
}

diesel_text_enum! {
    pub enum AttendanceStatus {
        Present,
        Absent,
        Late,
        Excused,
        HalfDay,
        SchoolBusiness,
    }
}

diesel_text_enum! {
    pub enum DayType {
        Working,
        Holiday,
        Weekend,
        SpecialEvent,
    }
}

diesel_text_enum! {
    pub enum ParticipantType {
        Participant,
        Organizer,
        Supervisor,
        Detained,
    }
}

diesel_text_enum! {
    pub enum SuspicionFlag {
        None,
        FrequentExit,
        Avoidance,
        UnusualDrowsiness,
        SkippingAfterInterval,
        Other,
    }
}

diesel_text_enum! {
    pub enum DetailedStatus {
        Normal,
        SickBay,
        FieldTrip,
        Counseling,
        Suspended,
        ExternalExam,
    }
}

diesel_text_enum! {
    pub enum ExcuseType {
        Medical,
        Educational,
        Family,
        Bereavement,
        Official,
    }
}

diesel_text_enum! {
    pub enum SubstitutionStatus {
        Pending,
        Confirmed,
        Completed,
        Cancelled,
    }
}

diesel_text_enum! {
    pub enum PreApprovedReason {
        Sick,
        FamilyEvent,
        Visa,
        Bereavement,
        Religious,
        Other,
        Medical,
    }
}

diesel_text_enum! {
    pub enum EmergencyStatus {
        Safe,
        Missing,
        Unknown,
        Injured,
    }
}

diesel_text_enum! {
    pub enum ExitReason {
        Medical,
        Personal,
        Disciplinary,
        Dismissal,
        FamilyEvent,
        Other,
    }
}

diesel_text_enum! {
    pub enum PolicyRuleType {
        ConsecutiveLate,
        TotalLate,
        UnexcusedAbsent,
    }
}

diesel_text_enum! {
    pub enum LeaveStatus {
        Pending,
        Approved,
        Rejected,
    }
}

diesel_text_enum! {
    pub enum StudentStatus {
        Active,
        Transferred,
        Graduated,
        Withdrawn,
        Suspended,
    }
}

diesel_text_enum! {
    pub enum Gender {
        Male,
        Female,
        Other,
    }
}

diesel_text_enum! {
    pub enum Religion {
        Buddhism,
        Hinduism,
        Islam,
        Christianity,
        Other,
    }
}

diesel_text_enum! {
    pub enum Ethnicity {
        Sinhala,
        Tamil,
        Muslim,
        Burger,
        Malay,
        Vedda,
        Other,
    }
}

diesel_text_enum! {
    pub enum EducationLevel {
        Primary,
        JuniorSecondary,
        SeniorSecondary,
        Collegiate,
    }
}

diesel_text_enum! {
    pub enum Medium {
        Sinhala,
        Tamil,
        English,
    }
}

diesel_text_enum! {
    pub enum FeeFrequency {
        Monthly,
        Quarterly,
        Annually,
        OneTime,
    }
}

diesel_text_enum! {
    pub enum PaymentMethod {
        Cash,
        BankTransfer,
        Cheque,
        Online,
    }
}

diesel_text_enum! {
    pub enum AllocationType {
        Student,
        Teacher,
        Department,
        Class,
    }
}

diesel_text_enum! {
    pub enum MaintenanceStatus {
        Pending,
        InProgress,
        Completed,
        Cancelled,
    }
}

diesel_text_enum! {
    pub enum TransactionType {
        Received,
        Spent,
    }
}

diesel_text_enum! {
    pub enum ComponentType {
        Allowance,
        Deduction,
        Bonus,
    }
}

diesel_text_enum! {
    pub enum PermissionSeverity {
        Low,
        Medium,
        High,
        Severe,
    }
}

diesel_text_enum! {
    pub enum PermissionEnum {
        UserCreate,
        UserRead,
        UserUpdate,
        UserDelete,
        UserManage,
        UserManageRoles,
        UserManagePermissions,
        RoleCreate,
        RoleRead,
        RoleUpdate,
        RoleDelete,
        RoleManage,
        RoleAssignPermissions,
        PermissionCreate,
        PermissionRead,
        PermissionUpdate,
        PermissionDelete,
        PermissionManage,
        PermissionSetManage,
        StaffCreate,
        StaffRead,
        StaffUpdate,
        StaffDelete,
        StaffManage,
        StaffManageAttendance,
        StaffManageLeaves,
        StudentCreate,
        StudentRead,
        StudentUpdate,
        StudentDelete,
        StudentManage,
        StudentManageGuardians,
        StudentManageEnrollment,
        StudentManageAttendance,
        StudentManageMarks,
        AcademicYearManage,
        TermManage,
        GradeLevelManage,
        ClassManage,
        SubjectManage,
        ClassSubjectTeacherManage,
        TimetableManage,
        ExamTypeManage,
        ExamManage,
        ExamSubjectManage,
        GradingSchemeManage,
        GradingCriterionManage,
        LibraryManage,
        CoCurricularManage,
        ResourceCreate,
        ResourceRead,
        ResourceUpdate,
        ResourceDelete,
        ResourceBook,
        ResourceViewBookings,
        CurriculumCreate,
        CurriculumRead,
        CurriculumUpdate,
        CurriculumDelete,
        CurriculumManage,
        SyllabusCreate,
        SyllabusRead,
        SyllabusUpdate,
        SyllabusDelete,
        SyllabusManage,
        MessagingCreate,
        MessagingRead,
        MessagingUpdate,
        MessagingDelete,
        MessagingSend,
        MessagingManage,
        BehaviorIncidentTypeCreate,
        BehaviorIncidentTypeRead,
        BehaviorIncidentTypeUpdate,
        BehaviorIncidentTypeDelete,
        BehaviorIncidentRecord,
        BehaviorIncidentRead,
        BehaviorIncidentUpdate,
        BehaviorIncidentDelete,
        ViewFinancialReports,
        SystemAdmin,
        UserUpdateMedium,
        UserDeleteSevere,
    }
}

impl PermissionEnum {
    pub fn severity(&self) -> PermissionSeverity {
        match self {
            PermissionEnum::UserDelete
            | PermissionEnum::RoleDelete
            | PermissionEnum::PermissionDelete
            | PermissionEnum::StaffDelete
            | PermissionEnum::StudentDelete
            | PermissionEnum::ResourceDelete
            | PermissionEnum::CurriculumDelete
            | PermissionEnum::SyllabusDelete
            | PermissionEnum::MessagingDelete
            | PermissionEnum::BehaviorIncidentTypeDelete
            | PermissionEnum::BehaviorIncidentDelete
            | PermissionEnum::UserDeleteSevere => PermissionSeverity::Severe,

            PermissionEnum::UserUpdate
            | PermissionEnum::RoleUpdate
            | PermissionEnum::PermissionUpdate
            | PermissionEnum::StaffUpdate
            | PermissionEnum::StudentUpdate
            | PermissionEnum::AcademicYearManage
            | PermissionEnum::TermManage
            | PermissionEnum::GradeLevelManage
            | PermissionEnum::ClassManage
            | PermissionEnum::SubjectManage
            | PermissionEnum::ClassSubjectTeacherManage
            | PermissionEnum::TimetableManage
            | PermissionEnum::ExamTypeManage
            | PermissionEnum::ExamManage
            | PermissionEnum::ExamSubjectManage
            | PermissionEnum::GradingSchemeManage
            | PermissionEnum::GradingCriterionManage
            | PermissionEnum::LibraryManage
            | PermissionEnum::UserUpdateMedium
            | PermissionEnum::PermissionSetManage
            | PermissionEnum::UserManageRoles
            | PermissionEnum::UserManagePermissions
            | PermissionEnum::RoleAssignPermissions
            | PermissionEnum::StaffManageAttendance
            | PermissionEnum::StaffManageLeaves
            | PermissionEnum::StudentManageGuardians
            | PermissionEnum::StudentManageEnrollment
            | PermissionEnum::StudentManageAttendance
            | PermissionEnum::StudentManageMarks
            | PermissionEnum::ResourceUpdate
            | PermissionEnum::ResourceBook
            | PermissionEnum::ResourceViewBookings
            | PermissionEnum::CurriculumUpdate
            | PermissionEnum::CurriculumManage
            | PermissionEnum::SyllabusUpdate
            | PermissionEnum::SyllabusManage
            | PermissionEnum::MessagingUpdate
            | PermissionEnum::MessagingSend
            | PermissionEnum::MessagingManage
            | PermissionEnum::BehaviorIncidentTypeUpdate
            | PermissionEnum::BehaviorIncidentRecord
            | PermissionEnum::BehaviorIncidentUpdate => PermissionSeverity::Medium,

            PermissionEnum::UserRead
            | PermissionEnum::RoleRead
            | PermissionEnum::PermissionRead
            | PermissionEnum::StaffRead
            | PermissionEnum::StudentRead
            | PermissionEnum::ResourceCreate
            | PermissionEnum::ResourceRead
            | PermissionEnum::CurriculumCreate
            | PermissionEnum::CurriculumRead
            | PermissionEnum::SyllabusCreate
            | PermissionEnum::SyllabusRead
            | PermissionEnum::MessagingCreate
            | PermissionEnum::MessagingRead
            | PermissionEnum::BehaviorIncidentTypeCreate
            | PermissionEnum::BehaviorIncidentTypeRead
            | PermissionEnum::BehaviorIncidentRead
            | PermissionEnum::ViewFinancialReports => PermissionSeverity::Low,

            _ => PermissionSeverity::Medium,
        }
    }
}
