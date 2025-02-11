use chrono::{Utc, DateTime};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum LogData {
    Int(i64),
}


#[derive(Debug, Clone)]
pub struct ChronoDB {
    data: VecDeque<(DateTime<Utc>, LogData)>, // (timestamp, value)
    max_size: usize,
}

impl ChronoDB {
    pub fn new(max_size: usize) -> Self {
        ChronoDB {
            data: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn insert(&mut self, value: LogData) {
        let timestamp = Utc::now();

        if self.data.len() == self.max_size {
            self.data.pop_front();
        }

        self.data.push_back((timestamp, value));
    }

    pub fn latest(&self) -> Option<(DateTime<Utc>, LogData)> {
        self.data.back().cloned()
    }

    pub fn get_all(&self) -> Vec<(DateTime<Utc>, LogData)> {
        self.data.clone().into()
    }
}

fn main() {
    let mut db = ChronoDB::new(10);

    db.insert(LogData::Int(42));

    if let Some(latest) = db.latest() {
        println!("Latest data point: {:?} at {}", latest.1, latest.0);
    }

    let all_logs = db.get_all();
    for (timestamp, log) in all_logs {
        println!("Log: {:?} at {}", log, timestamp);
    }
}


