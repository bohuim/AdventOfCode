use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use std::collections::HashSet;

// Part 1
static USIZE_LOWERCASE_A: usize = ('a' as usize);

fn two_or_three(word: &String) -> (u32, u32)
{
    let mut two_of:u32 = 0;
    let mut three_of: u32 = 0;

    // Array of counts for each letter.
    let mut letter_counts: [u32; 26] = [0; 26];
    for ch in word.chars() {
        letter_counts[ (ch as usize) - USIZE_LOWERCASE_A ] += 1;
    }

    for count in letter_counts.iter() {
        if *count == 2 { two_of = 1 }
        if *count == 3 { three_of = 1 }
    }

    return (two_of, three_of);
}

// Part 2
type IndexedID = HashSet< (u8, char) >;

fn index_id(id: &String) -> IndexedID 
{
    let mut set = IndexedID::new();
    for (i, ch) in id.chars().enumerate() {
        set.insert( (i as u8, ch) );
    }
    set
}

fn main()
{
    let name = Path::new("2.input");
    let file = File::open(name).expect("file not found");

    let box_ids: Vec<String> = BufReader::new(file).lines().filter_map(Result::ok).collect();

    // Part 1
    let (two, three) = box_ids.iter().map(two_or_three).fold((0,0), |a,b| (a.0 + b.0, a.1 + b.1));
    println!("Part 1: {:#?}", two * three); 

    // Part 2
    let sets: Vec<IndexedID> = box_ids.iter().map(index_id).collect();
    for (i, set1) in sets.iter().enumerate()
    {
        for (j, set2) in sets.iter().enumerate()
        {
            let diff: HashSet<_> = set1.difference(&set2).collect();
            if diff.len() == 1 {
                let mut same: Vec<_> = set1.intersection(&set2).collect();

                // Sort the vector by the letter positions, then collect into a string.
                same.sort();
                let letters: String = same.iter().map(|&(_, ch)| ch).cloned().collect();

                println!("Part 2");
                println!("[{:?}] {:?}", i, box_ids[i]);
                println!("[{:?}] {:?}", j, box_ids[j]);
                println!("diff {:?}", diff);
                println!("same {:#?}", letters);
                return;
            }
        }
    }
}

