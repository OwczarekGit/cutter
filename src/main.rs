use std::{error::Error, process::Command};



fn main() {

    let ts1 = TimeStamp::from_str("00:00:10.0000").unwrap();
    let ts2 = TimeStamp::from_str("00:00:20.0000").unwrap();

    let mut cutter = Cutter::new(String::from("audio.mp3"), vec![ts1, ts2]);
    cutter.next();
    cutter.next();
    cutter.next();

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
            cmd.output()?;
            
            Ok(())
    }
}

#[derive(Debug)]
struct TimeStamp{
    hour:   u32,
    minute: u32,
    second: u32,
    milis:  u32,
}

impl TimeStamp {

    pub fn get(&self) -> String {
        format!("{}:{}:{}.{}", self.hour, self.minute, self.second, self.milis)
    }

    pub fn from_str(input: &str) -> Result<Self, Box<dyn Error>>{

        let mut split = input.split(".");
        let t1 = split.next().ok_or("")?;
        let ms = split.next().ok_or("0000")?;

        let milis: u32 = ms.parse()?;
        let mut split2 = t1.split(":");

        let hour = if let Some(v1) = split2.next() {
            v1.parse::<u32>()?
        } else{
            0u32
        };
        
        let minute = if let Some(v1) = split2.next() {
            v1.parse::<u32>()?
        } else{
            0u32
        };
        
        let second = if let Some(v1) = split2.next() {
            v1.parse::<u32>()?
        } else{
            0u32
        };

        Ok(Self { 
            hour,
            minute,
            second,
            milis
        })
    }
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
    assert_eq!(ts.milis, 123u32);
}

#[test]
fn parse_incorrectly_from_string(){
    let input = "0q1:02e:52f.012s3";
    
    let ts = TimeStamp::from_str(input);

    assert!(ts.is_err());
}

#[test]
fn parse_incorrectly_from_partially_valid_string(){
    let input = "02:32.1234";
    
    let ts = Timestamp::from_str(input);

    assert!(ts.is_ok());

    let ts = ts.unwrap();

    assert_eq!(ts.hour, 2);
    assert_eq!(ts.minute, 32);
    assert_eq!(ts.second, 0);
    assert_eq!(ts.milis, 1234);
}