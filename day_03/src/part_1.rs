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

    
    let mut first_number_largest: i32 = 0;
    let mut second_number_largest: i32 = 0;

    for (i,number) in battery_bank.chars().map(|d| d.to_digit(10)).enumerate(){
        let num: i32 = number.expect("failed parsing bank to digit") as i32;

        if i == 0{
            first_number_largest = num;
        }else if i!= battery_bank.len()-1{

            if num > first_number_largest {
                first_number_largest = num;
                second_number_largest = 0;
            }else if num>second_number_largest {
                second_number_largest = num;

            }
        }else{
            if num>second_number_largest {
                second_number_largest = num;
            }
        }

    }

    let tmp: String = first_number_largest.to_string() + &second_number_largest.to_string();
    let tmp: i32 = tmp.parse::<i32>().expect("failed to parse number to int");
    return tmp

}
    
#[cfg(test)]
mod tests;