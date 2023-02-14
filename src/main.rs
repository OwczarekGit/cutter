use std::{error::Error, process::Command};
use std::cmp::Ordering;

use clap::Parser;


#[derive(Debug, Parser, Clone)]
pub struct Config {
    input: String,
    extension: String,
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

    let mut cutter = Cutter::new(config.input, timestamps, config.extension);
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
                format!("{}.{}", self.index, self.extension).as_str()
            ).unwrap();

            self.prev = next.get();
            self.timestamps.remove(0);
            self.index += 1;
            return Some(());
        }

        Self::extract_part_to_end(
            self.input.as_str(),
            self.prev.as_str(),
            format!("{}.{}", self.index, self.extension).as_str()
        ).unwrap();

        None
    }
}

#[derive(Debug)]
pub struct Cutter {
    timestamps: Vec<TimeStamp>,
    index: u32,
    extension: String,
    input: String,
    prev: String
}

impl Cutter {
    fn new(input: String, timestamps: Vec<TimeStamp>, extension: String) -> Self {
        Self { input, timestamps, index: 1u32, prev: String::from("00:00:00.0000"), extension }
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
            cmd.output()?;
            
            Ok(())
    }

    fn extract_part_to_end(input: &str, from: &str, output: &str) -> Result<(), Box<dyn Error>>{
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-ss", from,
            "-i", input,
            output
        ]);

        println!("{cmd:?}");
        cmd.output()?;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeStamp{
    hour:   u32,
    minute: u32,
    second: u32,
    millis:  u32,
}

impl TimeStamp {

    /// Returns timestamp in format HH:MM:SS.MS trimming leading zeros.
    ///
    /// # Example
    ///
    /// ```
    /// let x = TimeStamp::from_str("00:21:37.0123");
    /// assert_eq!(x.get(), "0:21:37.123");
    /// ```
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

        let hours_field = split2.next();
        let minutes_field = split2.next();
        let seconds_field = split2.next();


        // println!("HF: {hours_field:?}");
        // println!("MF: {minutes_field:?}");
        // println!("SF: {seconds_field:?}");

        let mut hour = "0";
        let mut minute = "0";
        let mut second = "0";

        if hours_field.is_none() {
            if minutes_field.is_none() {
                second = seconds_field.ok_or("00")?;
            } else {
                minute = minutes_field.ok_or("00")?;
                second = seconds_field.ok_or("00")?;
            }
        } else {
            if minutes_field.is_none() {
                second = hours_field.ok_or("00")?;
            } else {
                if seconds_field.is_none() {
                    minute = hours_field.ok_or("00")?;
                    second = minutes_field.ok_or("00")?;
                } else {
                    hour = hours_field.ok_or("00")?;
                    minute = minutes_field.ok_or("00")?;
                    second = seconds_field.ok_or("00")?;
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
