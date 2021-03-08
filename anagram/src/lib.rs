use std::collections::HashSet;
use std::borrow::{Borrow, BorrowMut};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for &possible_anagram in possible_anagrams.iter() {
       let result = str::as_bytes(&possible_anagram.to_lowercase());

        println!("{:?}", result);

    }
    anagrams

}
