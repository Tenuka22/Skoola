use chrono::{NaiveDateTime, DateTime};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct CustomFaker;

impl CustomFaker {
    pub fn date_time_between(start_date: NaiveDateTime, end_date: NaiveDateTime) -> NaiveDateTime {
        let mut rng = rand::thread_rng();
        let start_timestamp = start_date.and_utc().timestamp();
        let end_timestamp = end_date.and_utc().timestamp();
        let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
        DateTime::from_timestamp(random_timestamp, 0).unwrap().naive_utc()
    }

    pub fn pick_from_vec<T: Clone>(items: &[T]) -> T {
        let mut rng = rand::thread_rng();
        items.choose(&mut rng).unwrap().clone()
    }
}