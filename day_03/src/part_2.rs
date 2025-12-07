use core::num;
use std::f64::DIGITS;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::os::windows::thread;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    
    let mut score: i64 = 0;

    for (c,line) in buf_reader.lines().enumerate() {
        let bank:String = line.expect("file read error");
        println!("On line: {:?}", c);
        score+=parse_bank(bank);
        
    }
   
     
    println!("done. {}", score);
    Ok(())



}

fn parse_bank(battery_bank: String)->i64{

   
    let length = battery_bank.len();
    let mut first_number_largest: u32 = 0;
    let mut start_pos: usize =0;
    let mut curr_highest: i64 =0;
    let battery_int_list: Vec<u32> = battery_bank.chars().map(|d| d.to_digit(10).unwrap()).collect();
    
       
    
    
    
    //1
    for t in 0..length-11{
        if battery_int_list[t] > first_number_largest{
            first_number_largest=battery_int_list[t]
        }
        if battery_int_list[t] == first_number_largest{

        
        //2
            for i in t+1..length-10{
                //3
                for j in i+1..length-9{
                    if j!=i{
                        //4
                        for k in j+1..length-8{
                            if k!=j && k!=i{
                                //5
                                for l in k+1..length-7{
                                    if l!=j && l!=i&& l!=k{
                                        //6
                                        for m in l+1..length-6{
                                            if m!=j && m!=i && m!=k && m!=l{
                                                //7
                                                for n in m+1..length-5{
                                                    if n!=j && n!=i && n!=k && n!=l && n!=m{
                                                        //8
                                                        for o in n+1..length-4{
                                                            if o!=j && o!=i && o!=k && o!=l && o!=m && o!=n{
                                                                //9
                                                                for p in o+1..length-3{
                                                                    if p!=j && p!=i && p!=k && p!=l && p!=m && p!=n && p!=o{
                                                                        //10
                                                                        for q in p+1..length-2{
                                                                            if q!=j && q!=i && q!=k && q!=l && q!=m && q!=n && q!=o && q!=p{
                                                                                //11
                                                                                for r in q+1..length-1{
                                                                                    if r!=j && r!=i && r!=k && r!=l && r!=m && r!=n && r!=o && r!=p && r!=q{
                                                                                        //12
                                                                                        for s in r+1..length{
                                                                                            if s!=j && s!=i && s!=k && s!=l && s!=m && s!=n && s!=o && s!=p && s!=q && s!=r{
                                                                                                //get unique combinations of letters for whats left
                                                                                                let tmp=battery_int_list[t].to_string() + &battery_int_list[i].to_string() + &battery_int_list[j].to_string() + &battery_int_list[k].to_string() + &battery_int_list[l].to_string()+ &battery_int_list[m].to_string()  + &battery_int_list[n].to_string() + &battery_int_list[o].to_string() + &battery_int_list[p].to_string() + &battery_int_list[q].to_string()+ &battery_int_list[r].to_string() + &battery_int_list[s].to_string() ;
                                                                                                let tmp = tmp.parse::<i64>().expect("cant make number");
                                                                                                if tmp>curr_highest{
                                                                                                    curr_highest=tmp;
                                                                                                    //println!("new best: {:?}", tmp);
                                                                                                }
                                                                                                
                                                                                                

                                                                                            }
                                                                                        }

                                                                                    }
                                                                                }

                                                                            }
                                                                        }

                                                                    }
                                                                }

                                                            }
                                                        }

                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                }

            }
        }
    }
    println!("outcomes: {:?}",curr_highest);
    return curr_highest

}
    
#[cfg(test)]
mod tests;