use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    let mut current_dial_read: i32 = 50;
    let mut score: i32 = 0;

    for line in buf_reader.lines() {
        let content = line.expect("oops file error");
        let tmp: Vec<char> = content.chars().collect();
        
        let letter: char = tmp[0];
        let tmp: String = tmp[1..].iter().collect();
        let num: i32 = tmp.parse::<i32>().expect("error converting chars back to number");

        //println!("Rule is {}, {}", letter,num);
        if letter == 'R' {

            current_dial_read +=  num  
        } else if letter == 'L' {
            current_dial_read -=  num
        }else{
            println!("Your letters fucked up")
        }

        while current_dial_read < 0 || current_dial_read > 99{
            if current_dial_read > 99{
                current_dial_read = current_dial_read - 100;
                //score+=1;
            }else if current_dial_read < 0  {
                current_dial_read = current_dial_read + 100;
                //score+=1;
            }else if current_dial_read==0{
                score+=1;
            }    
        }
        

        //println!("condition is now: {}", current_dial_read);
        if current_dial_read==0{
            score+=1;
        }
    }
   
     
    println!("done. {}", score);
    Ok(())

}
