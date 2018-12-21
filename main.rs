use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::iter::FromIterator;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];
    println!("TextStat. Reading from:: {}", filename);
    // Complete the code here!
    
    let mut result: BTreeMap<String, isize> = BTreeMap::new();
    let reader = BufReader::new(File::open(filename).expect("Cannot open file.txt"));
    let mut sum_words = 0;
    let mut length = 0.0;
    let mut v_len = vec![0; 500];

    for line in reader.lines() {
    	for words in line.unwrap().to_lowercase().split_whitespace() {
    		for vol in words.split(|x|( x == ';' || x == ',' || x == '.' 
                                      || x == '”'||  x == '?' || x == '“'
                                      || x == '!' || x == '-' || x ==  '\r' 
                                      || x == '\n'|| x == ' ' || x == '‘' 
                                      || x == '_' || x == '﻿' || x == '(' 
                                      || x == ')' || x == '#' || x == '#' 
                                      || x == '%' || x == '$' || x == '1' 
                                      || x == '2' || x == '3' || x == '4'
                                      || x == '5' || x == '6' || x == '7' 
                                      || x == '8' || x == '9' || x == '0' 
                                      || x == '*' || x == '\\' || x == '['
                                      || x == ']' || x == '/'|| x == ':' 
                                      || x == '—'|| x == '†' || x == '•' 
                                      || x == '"' || x == '+'  || x == '&'
                                      || x == '°' || x == '£' )) {
            
                            length = length + vol.len() as f64;
                            v_len[vol.len()] += 1;
                            *(result.entry(vol.to_string()).or_insert(0)) += 1;
                      }
           sum_words += 1;
           }
    }
    let average = length / sum_words as f64;
     
    println!("\nThe average word size is {}.\n", average);
    println!("The total number of words is {}.\n", sum_words);
    for x in 1..11 {
         println!("The size of {} word appears {} times.\n", x, v_len[x]);
    }
    let mut v = Vec::from_iter(result);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    println!("The 10 most used words are: as a format of (\"words\" , frequency)");
    for i in 1..11{
        println!( "{:?}" ,v[i]);
    }
     
}
