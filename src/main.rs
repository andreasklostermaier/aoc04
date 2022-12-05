use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut full_overlaps    = 0u32;
    let mut partial_overlaps = 0u32;

    if let Ok(lines) = read_lines("./data/input.txt") {

        for pair in lines.flatten() {

            let elves:  Vec<&str> = pair.split(',').collect();
            let range1: Vec<&str> = elves[0].split('-').collect();
            let range2: Vec<&str> = elves[1].split('-').collect();
            let rs1 = range1[0].parse::<u32>().unwrap();
            let re1 = range1[1].parse::<u32>().unwrap();
            let rs2 = range2[0].parse::<u32>().unwrap();
            let re2 = range2[1].parse::<u32>().unwrap();
            
            // check for full overlap
            if (rs2 >= rs1 && re2 <= re1) || (rs1 >= rs2 && re1 <= re2) {
               full_overlaps += 1;
            }

            // check for partial overlap
            if ( (rs1..=re1).contains(&rs2) ) ||
               ( (rs1..=re1).contains(&re2) ) ||
               ( (rs2..=re2).contains(&rs1) ) ||
               ( (rs2..=re2).contains(&re1) ) {
                partial_overlaps += 1;
            }

        }
    }

    // Part 1
    println!("Part 1");
    println!("{} ranges overlap fully.", full_overlaps);

    // Part 2
    println!("Part 2");
    println!("{} ranges overlap partially.", partial_overlaps);
 
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
