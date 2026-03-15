use crate::schema::{
    club_activities, club_members, clubs, competition_participants, competitions,
    cultural_event_participants, cultural_events, sport_event_participants, sport_team_members, sport_teams, sports, student_achievements,
};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// --- Sports ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = sports)]
pub struct Sport {
    pub id: String,
    pub sport_name: String,
    pub description: Option<String>,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = sports)]
pub struct CreateSportRequest {
    pub sport_name: String,
    pub description: Option<String>,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = sports)]
pub struct UpdateSportRequest {
    pub sport_name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = sport_teams)]
pub struct SportTeam {
    pub id: String,
    pub sport_id: String,
    pub team_name: String,
    pub grade_level: String,
    pub coach_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = sport_teams)]
pub struct CreateSportTeamRequest {
    pub sport_id: String,
    pub team_name: String,
    pub grade_level: String,
    pub coach_id: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = sport_teams)]
pub struct UpdateSportTeamRequest {
    pub team_name: Option<String>,
    pub grade_level: Option<String>,
    pub coach_id: Option<String>,
}

// --- Clubs ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = clubs)]
pub struct Club {
    pub id: String,
    pub club_name: String,
    pub description: Option<String>,
    pub teacher_in_charge_id: String,
    pub meeting_schedule: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = clubs)]
pub struct CreateClubRequest {
    pub club_name: String,
    pub description: Option<String>,
    pub teacher_in_charge_id: String,
    pub meeting_schedule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = clubs)]
pub struct UpdateClubRequest {
    pub club_name: Option<String>,
    pub description: Option<String>,
    pub teacher_in_charge_id: Option<String>,
    pub meeting_schedule: Option<String>,
}

// --- Competitions ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = competitions)]
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

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = competitions)]
pub struct CreateCompetitionRequest {
    pub competition_name: String,
    pub competition_type: String,
    pub date: NaiveDateTime,
    pub organizer: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = competitions)]
pub struct UpdateCompetitionRequest {
    pub competition_name: Option<String>,
    pub competition_type: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub organizer: Option<String>,
    pub level: Option<String>,
}

// --- Student Achievements ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = student_achievements)]
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

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = student_achievements)]
pub struct CreateStudentAchievementRequest {
    pub student_id: String,
    pub achievement_type: String,
    pub description: String,
    pub date: NaiveDate,
    pub certificate_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_achievements)]
pub struct UpdateStudentAchievementRequest {
    pub achievement_type: Option<String>,
    pub description: Option<String>,
    pub date: Option<NaiveDate>,
    pub certificate_url: Option<String>,
}

// --- Cultural Events ---

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = cultural_events)]
pub struct CulturalEvent {
    pub id: String,
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = cultural_events)]
pub struct CreateCulturalEventRequest {
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = cultural_events)]
pub struct UpdateCulturalEventRequest {
    pub event_name: Option<String>,
    pub event_date: Option<NaiveDateTime>,
    pub venue: Option<String>,
    pub description: Option<String>,
}

// Additional necessary structs
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(primary_key(team_id, student_id))]
#[diesel(table_name = sport_team_members)]
pub struct SportTeamMember {
    pub team_id: String,
    pub student_id: String,
    pub position: Option<String>,
    pub joined_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(primary_key(event_id, student_id))]
#[diesel(table_name = sport_event_participants)]
pub struct SportEventParticipant {
    pub event_id: String,
    pub student_id: String,
    pub team_id: Option<String>,
    pub position: Option<String>,
    pub points: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(primary_key(club_id, student_id))]
#[diesel(table_name = club_members)]
pub struct ClubMember {
    pub club_id: String,
    pub student_id: String,
    pub role: String,
    pub joined_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(table_name = club_activities)]
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

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(primary_key(competition_id, student_id))]
#[diesel(table_name = competition_participants)]
pub struct CompetitionParticipant {
    pub competition_id: String,
    pub student_id: String,
    pub position: Option<String>,
    pub award: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub representing_type: Option<String>,
    pub representing_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone, JsonSchema, ApiComponent)]
#[diesel(primary_key(event_id, student_id))]
#[diesel(table_name = cultural_event_participants)]
pub struct CulturalEventParticipant {
    pub event_id: String,
    pub student_id: String,
    pub performance_type: String,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentCoCurricularSummary {
    pub sports: Vec<SportTeamMember>,
    pub clubs: Vec<ClubMember>,
    pub achievements: Vec<StudentAchievement>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddSportTeamMemberRequest {
    pub student_id: String,
    pub position: Option<String>,
    pub joined_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordEventResultRequest {
    pub student_id: String,
    pub team_id: Option<String>,
    pub position: Option<String>,
    pub points: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddClubMemberRequest {
    pub student_id: String,
    pub role: String,
    pub joined_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateClubActivityRequest {
    pub club_id: String,
    pub activity_name: String,
    pub activity_date: NaiveDateTime,
    pub description: Option<String>,
    pub participants_count: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = club_activities)]
pub struct UpdateClubActivityRequest {
    pub club_id: Option<String>,
    pub activity_name: Option<String>,
    pub activity_date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub participants_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddCompetitionParticipantRequest {
    pub student_id: String,
    pub position: Option<String>,
    pub award: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AddCulturalEventParticipantRequest {
    pub student_id: String,
    pub performance_type: String,
    pub role: Option<String>,
}
