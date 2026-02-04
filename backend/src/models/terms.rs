use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use crate::schema::terms;
use chrono::NaiveDate;

#[derive(Queryable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = terms)]
pub struct Term {
    pub id: String, // Changed from i32 to String
    pub academic_year_id: String, // Changed from i32 to String
    pub term_number: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = terms)]
pub struct CreateTermRequest {
    pub academic_year_id: String, // Changed from i32 to String
    pub term_number: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = terms)]
pub struct UpdateTermRequest {
    pub academic_year_id: Option<String>, // Changed from Option<i32> to Option<String>
    pub term_number: Option<i32>,
    pub name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct TermResponse {
    pub id: String, // Changed from i32 to String
    pub academic_year_id: String, // Changed from i32 to String
    pub term_number: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Term> for TermResponse {
    fn from(term: Term) -> Self {
        TermResponse {
            id: term.id,
            academic_year_id: term.academic_year_id,
            term_number: term.term_number,
            name: term.name,
            start_date: term.start_date,
            end_date: term.end_date,
            created_at: term.created_at,
            updated_at: term.updated_at,
        }
    }
}