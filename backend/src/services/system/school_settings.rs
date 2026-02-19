use diesel::prelude::*;
use diesel::upsert::excluded;
use crate::{
    errors::APIError,
    AppState,
    models::system::setting::{SchoolSettingResponse, UpdateSchoolSettingRequest},
    database::tables::SchoolSetting,
};
use actix_web::web;
use chrono::Utc;
use crate::schema::school_settings;

pub async fn get_all_settings(
    pool: web::Data<AppState>,
) -> Result<Vec<SchoolSettingResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = school_settings::table.load::<SchoolSetting>(&mut conn)?;
    Ok(list.into_iter().map(|s| SchoolSettingResponse {
        setting_key: s.setting_key,
        setting_value: s.setting_value,
        description: s.description,
        updated_at: s.updated_at,
    }).collect())
}

pub async fn update_setting(
    pool: web::Data<AppState>,
    key: String,
    req: UpdateSchoolSettingRequest,
) -> Result<SchoolSettingResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    diesel::insert_into(school_settings::table)
        .values(SchoolSetting {
            setting_key: key.clone(),
            setting_value: req.setting_value,
            description: req.description,
            updated_at: Utc::now().naive_utc(),
        })
        .on_conflict(school_settings::setting_key)
        .do_update()
        .set((
            school_settings::setting_value.eq(excluded(school_settings::setting_value)),
            school_settings::description.eq(excluded(school_settings::description)),
            school_settings::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    let updated = school_settings::table.find(&key).first::<SchoolSetting>(&mut conn)?;
    Ok(SchoolSettingResponse {
        setting_key: updated.setting_key,
        setting_value: updated.setting_value,
        description: updated.description,
        updated_at: updated.updated_at,
    })
}
