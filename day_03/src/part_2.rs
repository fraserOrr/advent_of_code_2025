use core::num;
use std::f64::DIGITS;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Battery{
    voltage: i32,
    position: usize
}
fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    
    let mut score: i32 = 0;

    for line in buf_reader.lines() {
        let bank:String = line.expect("file read error");
        score+=parse_bank(bank);
        
    }
   
     
    println!("done. {}", score);
    Ok(())



}

fn parse_bank(battery_bank: String)->i32{

    let outcomes: Vec<i32> = Vec::new();
    let length = battery_bank.len();
    let digit_length: usize = 12;
    for i in 0..digit_length-12{
        
    }

    
    return 0

}
    
#[cfg(test)]
mod tests;