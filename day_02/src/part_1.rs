use core::num;
use std::f64::DIGITS;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    
    let mut score: i64 = 0;

    for line in buf_reader.lines() {
        let content= line.expect("error reading file");
        let puzzle_input: Vec<&str> = content.split(",").collect();

        for input in puzzle_input.iter(){
            println!("{:?}",input);
            let content_2: Vec<&str> = input.split("-").collect();
            let start: i64 = content_2[0].parse::<i64>().unwrap();
            let end: i64 = content_2[1].parse::<i64>().unwrap();
            score+= return_invalid_id_count(start, end);
        }
        
    }
   
     
    println!("done. {}", score);
    Ok(())



}

fn return_invalid_id_count(start: i64, end: i64)->i64{
    let mut score: i64 = 0;
    
    for i in start..end+1{
       // println!("input: {:?}", i);
       if is_invalid_id(i)==false{
            score+=i;
       }
    }



    return score;

}

fn is_invalid_id(number: i64)->bool{
    let mut valid: bool = false;

    let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let length = digits.len();
    if digits.len()%2!=0{
        valid=true;
    }else{
        let part_1: &[u32] = &digits[0..length/2];
        let part_2: &[u32] = &digits[length/2..];
        //println!("before: {:?},{:?}", part_1, part_2);
        let mut number_1: i64 =0;
        let mut number_2: i64 =0;
        for (i,num) in part_1.iter().rev().enumerate(){

            let base: i64=10;
            number_1+= num.clone() as i64 * base.pow(i as u32);
        }
        for (i,num) in part_2.iter().rev().enumerate(){
           
            let base: i64=10;
            number_2+= num.clone() as i64 * base.pow(i as u32);
        }
        //println!("after: {:?},{:?}", number_1, number_2);
        
        if number_1 == number_2{
            valid=false;
        }else{
            valid=true;
        }
    }


    


    return valid
}

    
/* 
#[cfg(test)]
mod tests;*/