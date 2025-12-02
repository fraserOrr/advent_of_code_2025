
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::slice::*;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    
    let mut score: i64 = 0;

    for line in buf_reader.lines() {
        let content= line.expect("error reading file");
        let puzzle_input: Vec<&str> = content.split(",").collect();

        for input in puzzle_input.iter(){
            //println!("{:?}",input);
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
            //println!("invalid number: {}", i);
       }
    }



    return score;

}


fn is_invalid_id(number: i64)->bool{
    let mut valid: bool = true;

    let main_digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let length = main_digits.len();
   // println!("input: {}", number);
    if length>1{
        for i in 1..length/2+1{
            let mut sub_number_valid: bool = false;
            let mut digits: Chunks<'_, u32> = main_digits.chunks(i);
            ///println!("chunks: {:?}", digits);
            let base = digits.next().unwrap();
            ///println!("check base: {:?}", base);
            for k in 0..digits.len(){
                let next = digits.next().unwrap();
                ///println!("next: {:?}", next);
                if next != base{
                    sub_number_valid=true;
                }
            }
            
            
            if sub_number_valid==false{
                valid=false
            }
        }
    }
    


    


    return valid
}



#[cfg(test)]
mod tests;