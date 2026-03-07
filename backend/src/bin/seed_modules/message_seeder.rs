use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct MessageSeeder;

impl MessageSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for MessageSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Message module...");

        // 1. conversations
        println!("Seeding conversations...");
        for i in 0..10 {
            let id = next_id(conn, IdPrefix::CONVERSATION);
            insert_into(conversations::table)
                .values((
                    conversations::id.eq(id.clone()),
                    conversations::subject.eq(format!("Conversation {}", i)),
                    conversations::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            let u1 = get_random_id(&context.user_ids);
            insert_into(conversation_participants::table)
                .values((
                    conversation_participants::conversation_id.eq(id.clone()),
                    conversation_participants::user_id.eq(u1.clone()),
                ))
                .execute(conn)?;

            insert_into(messages::table)
                .values((
                    messages::id.eq(next_id(conn, IdPrefix::MESSAGE)),
                    messages::conversation_id.eq(id),
                    messages::sender_user_id.eq(u1),
                    messages::content.eq("Hello there!"),
                    messages::sent_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
