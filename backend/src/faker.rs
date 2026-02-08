use fake::{faker::*, Dummy, Fake, Faker};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct CustomFaker;

impl CustomFaker {
    pub fn date_time_between(start_date: NaiveDateTime, end_date: NaiveDateTime) -> NaiveDateTime {
        let mut rng = rand::thread_rng();
        let start_timestamp = start_date.timestamp();
        let end_timestamp = end_date.timestamp();
        let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
        NaiveDateTime::from_timestamp_opt(random_timestamp, 0).unwrap_or_else(|| Utc::now().naive_utc())
    }

    pub fn pick_from_vec<T: Clone>(items: &[T]) -> T {
        let mut rng = rand::thread_rng();
        items.choose(&mut rng).unwrap().clone()
    }
}