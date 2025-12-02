use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    let mut current_dial_read: i32 = 50;
    let mut score: i32 = 0;

    for line in buf_reader.lines() {
        let tmp_score;
        let content = line.expect("oops file error");
        let tmp: Vec<char> = content.chars().collect();
        
        let letter: char = tmp[0];
        let tmp: String = tmp[1..].iter().collect();
        let num: i32 = tmp.parse::<i32>().expect("error converting chars back to number");

        //println!("Rule is {}, {}", letter,num);
       (current_dial_read,tmp_score) = revolve(letter, current_dial_read, num);

       score+=tmp_score;
    }
   
     
    println!("done. {}", score);
    Ok(())

}

fn revolve(letter: char, current_dial: i32, num: i32)->(i32,i32){
    let mut local_score: i32 = 0;
    let mut new_dial: i32 = current_dial.clone();
    let mut rotations: i32 = 0;
    let mut first_turn: bool = true;
    if letter == 'R' {

        new_dial +=  num  
    } else if letter == 'L' {
        new_dial -=  num
    }else{
        println!("Your letters fucked up")
    }

    while new_dial < 0 || new_dial > 99{
        if new_dial > 99{
            new_dial = new_dial - 100;
            if new_dial != 0  {
                rotations+=1;
            }
            
            first_turn=false;
        }else if new_dial < 0  {
            new_dial = new_dial + 100;
            if new_dial == 0 && num < 99 {
                
            }else if first_turn==true && current_dial ==0{

            }else{
                rotations+=1;
            }

            first_turn=false;
        }   
    }
    
    

    //println!("condition is now: {}", current_dial_read);
    if new_dial==0{
        local_score+=1;
    }
    local_score+=rotations;
    return  (new_dial,local_score);

}


#[cfg(test)]
mod tests;