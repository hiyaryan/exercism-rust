// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_count= HashMap::with_capacity(magazine.len());
    let mut i: u32;
    for word in magazine {
        // Return the current count from the HashMap then increment it. 
        // If it doesn't exist initialize it to zero then increment it.
        magazine_word_count.entry(word).and_modify(|i| *i += 1).or_insert(1); 
    }

    // Main loop through the words in the note.
    for word in note {
        match magazine_word_count.get_mut(word) {
            // Return false if the word is in the magazine but does not contain enough occurences.
            Some(i) if *i > 0 => *i -= 1,
            // Return false if the word is not in the magazine.
            _ => return false,
        }
    }

    // Return true, the note is contained in the magazine.
    true
}