use std::{error::Error, process::Command};
use std::cmp::Ordering;

use clap::Parser;


#[derive(Debug, Parser, Clone)]
struct Config {
    input: String,
    timestamps: Vec<String>,
}

fn main() {
    let config: Config = Config::parse();
    // println!("{config:?}");

    let mut timestamps: Vec<TimeStamp> = config.timestamps.into_iter()
    .map(|s|{
        TimeStamp::from_str(s.as_str()).expect("Invalid timestamp")
    }).collect();

    timestamps.sort_by(|a,b| {
        let a_secs: f32 = (a.second + (a.minute * 60) + (a.hour * 60 * 60)) as f32 + (a.millis as f32 * 0.001);
        let b_secs: f32 = (b.second + (b.minute * 60) + (b.hour * 60 * 60)) as f32 + (b.millis as f32 * 0.001);

        return if a_secs > b_secs {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut cutter = Cutter::new(config.input, timestamps);
    while let Some(_) = cutter.next() {}
}

impl Iterator for Cutter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(next) = self.timestamps.first() {
            Self::extract_part(
                self.input.as_str(),
                self.prev.as_str(),
                &next.get(),
                format!("{}.mp3", self.index).as_str()
            ).unwrap();

            self.prev = next.get();
            self.timestamps.remove(0);
            self.index += 1;
            return Some(());
        }

        None
    }
}

#[derive(Debug)]
struct Cutter {
    timestamps: Vec<TimeStamp>,
    index: u32,
    input: String,
    prev: String
}

impl Cutter {
    fn new(input: String, timestamps: Vec<TimeStamp>) -> Self {
        Self { input, timestamps, index: 1u32, prev: String::from("00:00:00.0000") }
    }

    fn extract_part(input: &str, from: &str, to: &str, output: &str) -> Result<(), Box<dyn Error>>{
        let mut cmd = Command::new("ffmpeg");
            cmd.args([
                "-ss", from,
                "-to", to,
                "-i", input,
                output
            ]);

            println!("{cmd:?}");
            // cmd.output()?;
            
            Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct TimeStamp{
    hour:   u32,
    minute: u32,
    second: u32,
    millis:  u32,
}

impl TimeStamp {

    pub fn get(&self) -> String {
        format!("{}:{}:{}.{}", self.hour, self.minute, self.second, self.millis)
    }

    pub fn from_str(input: &str) -> Result<Self, Box<dyn Error>>{

        let mut split = input.split(".");
        let t1 = split.next().ok_or("00:00:00")?;

        let millis = if let Some(ms) = split.next(){
            ms
        } else {
            "0"
        };

        let mut split2 = t1.split(":");

        let mut hoursField = split2.next();
        let mut minutesField = split2.next();
        let mut secondsField = split2.next();


        // println!("HF: {hoursField:?}");
        // println!("MF: {minutesField:?}");
        // println!("SF: {secondsField:?}");

        let mut hour = "0";
        let mut minute = "0";
        let mut second = "0";

        if hoursField.is_none() {
            if minutesField.is_none() {
                second = secondsField.ok_or("00")?;
            } else {
                minute = minutesField.ok_or("00")?;
                second = secondsField.ok_or("00")?;
            }
        } else {
            if minutesField.is_none() {
                second = hoursField.ok_or("00")?;
            } else {
                if secondsField.is_none() {
                    minute = hoursField.ok_or("00")?;
                    second = minutesField.ok_or("00")?;
                } else {
                    hour = hoursField.ok_or("00")?;
                    minute = minutesField.ok_or("00")?;
                    second = secondsField.ok_or("00")?;
                }
            }
        }

        // println!("HOUR: {hour:?}");
        // println!("MIN: {minute:?}");
        // println!("SEC: {second:?}");
        // println!("MS: {milis:?}");

        let r = Self {
            hour: hour.parse::<u32>()?,
            minute: minute.parse::<u32>()?,
            second: second.parse::<u32>()?,
            millis: millis.parse::<u32>()?,
        };

        Ok(r)
    }
}


#[test]
fn parse_correctly_from_seconds_and_millis_only(){
    let input = "20.0342";

    let ts = TimeStamp::from_str(input);

    assert!(ts.is_ok());
    let ts = ts.unwrap();

    assert_eq!(ts.hour, 0u32);
    assert_eq!(ts.minute, 0u32);
    assert_eq!(ts.second, 20u32);
    assert_eq!(ts.millis, 342u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correctly_from_seconds_only(){
        let input = "20";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_ok());
        let ts = ts.unwrap();

        assert_eq!(ts.hour, 0u32);
        assert_eq!(ts.minute, 0u32);
        assert_eq!(ts.second, 20u32);
        assert_eq!(ts.millis, 0u32);
    }

    #[test]
    fn parse_correctly_from_string(){
        let input = "01:02:52.0123";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_ok());
        let ts = ts.unwrap();

        assert_eq!(ts.hour, 1u32);
        assert_eq!(ts.minute, 2u32);
        assert_eq!(ts.second, 52u32);
        assert_eq!(ts.millis, 123u32);
    }

    #[test]
    fn parse_correctly_from_string_without_leading_zeros(){
        let input = "4:1:12.23";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_ok());
        let ts = ts.unwrap();

        assert_eq!(ts.hour, 4u32);
        assert_eq!(ts.minute, 1u32);
        assert_eq!(ts.second, 12u32);
        assert_eq!(ts.millis, 23u32);
    }

    #[test]
    fn parse_incorrectly_from_string(){
        let input = "0q1:02e:52f.012s3";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_err());
    }

    #[test]
    fn parse_correctly_from_valid_string_without_hours(){
        let input = "02:32.1234";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_ok());

        let ts = ts.unwrap();

        assert_eq!(ts.hour, 0);
        assert_eq!(ts.minute, 2);
        assert_eq!(ts.second, 32);
        assert_eq!(ts.millis, 1234);
    }

    #[test]
    fn parse_correctly_from_valid_string_without_millis(){
        let input = "01:02:32";

        let ts = TimeStamp::from_str(input);

        assert!(ts.is_ok());

        let ts = ts.unwrap();

        assert_eq!(ts.hour, 1);
        assert_eq!(ts.minute, 2);
        assert_eq!(ts.second, 32);
        assert_eq!(ts.millis, 0);
    }
}
