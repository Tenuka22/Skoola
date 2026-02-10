use crate::schema::{
    sports, sport_teams, sport_team_members, sport_events, sport_event_participants,
    clubs, club_members, club_activities,
    competitions, competition_participants, student_achievements,
    cultural_events, cultural_event_participants
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, NaiveDate};
use schemars::JsonSchema;
use apistos::ApiComponent;

// --- Sports ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = sports)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sport {
    pub id: String,
    pub sport_name: String,
    pub description: Option<String>,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateSportRequest {
    pub sport_name: String,
    pub description: Option<String>,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = sport_teams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SportTeam {
    pub id: String,
    pub sport_id: String,
    pub team_name: String,
    pub grade_level: String,
    pub coach_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateSportTeamRequest {
    pub sport_id: String,
    pub team_name: String,
    pub grade_level: String,
    pub coach_id: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(primary_key(team_id, student_id))]
#[diesel(table_name = sport_team_members)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SportTeamMember {
    pub team_id: String,
    pub student_id: String,
    pub position: Option<String>,
    pub joined_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddSportTeamMemberRequest {
    pub student_id: String,
    pub position: Option<String>,
    pub joined_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = sport_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SportEvent {
    pub id: String,
    pub sport_id: String,
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub organizer: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateSportEventRequest {
    pub sport_id: String,
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub organizer: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(primary_key(event_id, student_id))]
#[diesel(table_name = sport_event_participants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SportEventParticipant {
    pub event_id: String,
    pub student_id: String,
    pub team_id: Option<String>,
    pub position: Option<String>,
    pub points: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordEventResultRequest {
    pub student_id: String,
    pub team_id: Option<String>,
    pub position: Option<String>,
    pub points: Option<i32>,
}

// --- Clubs ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = clubs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Club {
    pub id: String,
    pub club_name: String,
    pub description: Option<String>,
    pub teacher_in_charge_id: String,
    pub meeting_schedule: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateClubRequest {
    pub club_name: String,
    pub description: Option<String>,
    pub teacher_in_charge_id: String,
    pub meeting_schedule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(primary_key(club_id, student_id))]
#[diesel(table_name = club_members)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClubMember {
    pub club_id: String,
    pub student_id: String,
    pub role: String,
    pub joined_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddClubMemberRequest {
    pub student_id: String,
    pub role: String,
    pub joined_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = club_activities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClubActivity {
    pub id: String,
    pub club_id: String,
    pub activity_name: String,
    pub activity_date: NaiveDateTime,
    pub description: Option<String>,
    pub participants_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateClubActivityRequest {
    pub club_id: String,
    pub activity_name: String,
    pub activity_date: NaiveDateTime,
    pub description: Option<String>,
    pub participants_count: i32,
}

// --- Competitions ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = competitions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Competition {
    pub id: String,
    pub competition_name: String,
    pub competition_type: String,
    pub date: NaiveDateTime,
    pub organizer: String,
    pub level: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateCompetitionRequest {
    pub competition_name: String,
    pub competition_type: String,
    pub date: NaiveDateTime,
    pub organizer: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(primary_key(competition_id, student_id))]
#[diesel(table_name = competition_participants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CompetitionParticipant {
    pub competition_id: String,
    pub student_id: String,
    pub position: Option<String>,
    pub award: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddCompetitionParticipantRequest {
    pub student_id: String,
    pub position: Option<String>,
    pub award: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_achievements)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentAchievement {
    pub id: String,
    pub student_id: String,
    pub achievement_type: String,
    pub description: String,
    pub date: NaiveDate,
    pub certificate_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateStudentAchievementRequest {
    pub student_id: String,
    pub achievement_type: String,
    pub description: String,
    pub date: NaiveDate,
    pub certificate_url: Option<String>,
}

// --- Cultural Events ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = cultural_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CulturalEvent {
    pub id: String,
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateCulturalEventRequest {
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(primary_key(event_id, student_id))]
#[diesel(table_name = cultural_event_participants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CulturalEventParticipant {
    pub event_id: String,
    pub student_id: String,
    pub performance_type: String,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddCulturalEventParticipantRequest {
    pub student_id: String,
    pub performance_type: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentCoCurricularSummary {
    pub sports: Vec<SportTeamMember>,
    pub clubs: Vec<ClubMember>,
    pub achievements: Vec<StudentAchievement>,
}
