use chrono::{Datelike, NaiveDate};
use csv::Writer;
use rand::{distributions::Uniform, Rng};
use std::{error::Error, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let eigyosyo_list = vec!["宮崎", "福岡", "大阪", "東京"];
    let busyo_list = vec!["営業部", "総務部", "人事部", "経理部"];
    let start_date = NaiveDate::from_ymd(2023, 1, 1);
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(100, 999);

    let mut writer = Writer::from_path("sample_data.csv")?;
    writer.write_record(&["id", "eigyosyo", "busyo", "konnki_jisseki", "zennki_jisseki", "yosan", "recording_date" ])?;

    let mut id = 1;
    for month_offset in 0..=11 {
        let date = start_date.with_month((month_offset + 1) as u32).unwrap();
        let date_str = date.format("%Y/%m").to_string();
        for &eigyosyo in &eigyosyo_list {
            for &busyo in &busyo_list {
                writer.write_record(&[
                    id.to_string(),
                    eigyosyo.to_string(),
                    busyo.to_string(),
                    rng.sample(&range).to_string(),
                    rng.sample(&range).to_string(),
                    rng.sample(&range).to_string(),
                    date_str.clone(),
                ])?;
                id += 1;
            }
        }
    }

    writer.flush()?;
    Ok(())
}
