use anyhow::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use super::utils::{generate_uuid, random_datetime_in_past};
use super::{SeedModule, SeederContext};
use backend::config::Config;
use backend::models::messaging::{Conversation, ConversationParticipant, Message};
use backend::schema::{conversation_participants, conversations, messages};

pub struct MessageSeeder;

impl MessageSeeder {
    pub fn new() -> Self {
        MessageSeeder
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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding messages and conversations...");

        let mut rng = rand::thread_rng();

        if context.user_ids.len() < 2 {
            println!("Skipping message seeding: not enough users.");
            return Ok(());
        }

        let mut all_conversations = Vec::new();
        let mut all_participants = Vec::new();
        let mut all_messages = Vec::new();

        for i in 0..seed_count_config.conversations {
            let conv_id = generate_uuid();
            let subject = format!("General Discussion {}", i + 1);

            all_conversations.push(Conversation {
                id: conv_id.clone(),
                subject,
                created_at: random_datetime_in_past(1),
            });

            // Randomly select 2 to `conversation_participants_per_conversation` participants
            let num_participants = rng.gen_range(
                2..=context
                    .user_ids
                    .len()
                    .min(seed_count_config.conversation_participants_per_conversation),
            );
            let mut shuffled_users = context.user_ids.clone();
            shuffled_users.shuffle(&mut rng);
            let conv_participants = &shuffled_users[0..num_participants];

            for user_id in conv_participants {
                all_participants.push(ConversationParticipant {
                    conversation_id: conv_id.clone(),
                    user_id: user_id.clone(),
                });
            }

            // Create configurable number of messages per conversation
            let num_messages = rng.gen_range(5..=seed_count_config.messages_per_conversation);
            for j in 0..num_messages {
                let sender_id = conv_participants.choose(&mut rng).unwrap();
                all_messages.push(Message {
                    id: generate_uuid(),
                    conversation_id: conv_id.clone(),
                    sender_user_id: sender_id.clone(),
                    content: format!("Message {} in conversation {}", j + 1, i + 1),
                    sent_at: random_datetime_in_past(1),
                    read_at: if rng.gen_bool(0.7) {
                        Some(random_datetime_in_past(0))
                    } else {
                        None
                    },
                });
            }

            context.conversation_ids.push(conv_id);
        }

        diesel::insert_into(conversations::table)
            .values(&all_conversations)
            .execute(conn)?;

        diesel::insert_into(conversation_participants::table)
            .values(&all_participants)
            .execute(conn)?;

        diesel::insert_into(messages::table)
            .values(&all_messages)
            .execute(conn)?;

        println!(
            "Seeded {} conversations, {} participants, and {} messages.",
            all_conversations.len(),
            all_participants.len(),
            all_messages.len()
        );
        Ok(())
    }
}
