use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let file = File::open("input.txt")?;
    let  buf_reader = BufReader::new(file);
  
    let mut scroll_map:HashMap<(i32,i32), char>    = HashMap::new();
    

    
    let mut score: i32 = 0;
    
    
    for (y,line ) in buf_reader.lines().enumerate() {
        let char_list: Vec<char> = line.expect("file read error").chars().collect::<Vec<char>>();

        for (c,single_char) in char_list.iter().enumerate() {
            print!("{}", single_char);
            if single_char == &'@' {
                assert!(scroll_map.contains_key(&(c as i32,y as i32))==false);
            
                scroll_map.insert((c as i32,y as i32), *single_char);
            }
            
        }
        println!()
    }

    let mut round_score: i32=1;
    while round_score!=0 {
        (round_score, scroll_map) = calculate_map(&scroll_map);
        score += round_score;
    }
    
    

    
    println!("map context: {:?}", scroll_map);  
    println!("done. {}", score);
    Ok(())



}

fn calculate_map(scroll_map: &HashMap<(i32,i32), char>) -> (i32,HashMap<(i32,i32), char>) {
    let mut score: i32 = 0;
    let mut return_map: HashMap<(i32,i32), char> = HashMap::new();
    return_map.clone_from(scroll_map);
    for (scrolls_location,_scroll) in scroll_map.iter(){

        let (x,y) = scrolls_location;
        let mut adjacent_scrolls = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if scroll_map.contains_key(&(x + dx, y + dy)) {
                    
                    adjacent_scrolls += 1;
                    
                }
            }
        }

        if adjacent_scrolls <4 {
            //we can take this scroll out
            return_map.remove(&(*x,*y));
            score += 1;
        }
    }

    (score,return_map)
}
