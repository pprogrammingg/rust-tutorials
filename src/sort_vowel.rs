// Exercise: Implement a function that sorts vowels in a string 
// while keeping consonants in their original position.
// ✅ Preserve case while sorting vowels
// ✅ Work with Unicode vowels (e.g., á, é, î, ö, ü)
// ✅ Handle large input strings efficiently

use std::{char::ToLowercase, collections::HashSet};

pub fn sort_vowels(original_str: &str) -> String {
    // Implement sorting vowels while keeping consonants in place
    
    // initiate an empty String as result
    let mut result = String::from("");
    let original_string = original_str.to_owned();

    println!("\nOriginal String is {}", original_str);

    let valid_lower_case_vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u', 'á', 'é', 'î', 'ö', 'ü'].into_iter().collect();
    let mut vowels_found: Vec<char> = Vec::new();

    for (i, char) in original_string.chars().enumerate() {
        // if valid vowel
        // create two equal size vecs of vowels and indecies of 
        // the vowels in the original string (repeating chars stay also get added) 
        // println!("current index, char is (i, char) is ({}, {})", i, char);
        // println!("lower case of char {} is {}", char, &char.to_lowercase());
        if valid_lower_case_vowels.contains( &char.to_lowercase().next().unwrap()) {
            println!("vowel branch: (i, char) is ({}, {})", i, char);
            vowels_found.push(char.clone());
        }
    } 

    print!("vowels_from_string before sort {:?} \n", vowels_found);

    // sort the vowel_from_string
    vowels_found.sort();

    println!("vowels_from_string after sort {:?} \n", vowels_found);

    // loop on original string, and keep a separate counter 
    // to see which vowel to take from sorted vowel vec
    let mut counter = 0; 
    let mut result: Vec<char> = Vec::from([]);
    for (idx, char) in original_string.chars().enumerate(){
        // if encounter a vowel, choose a vowel from sorted list based on counter as index
        if valid_lower_case_vowels.contains( &char.to_lowercase().next().unwrap()) {
            result.push(*vowels_found.get(counter).unwrap());
            counter = counter + 1;
        }
        else {
            result.push(char);
        }
    }

    let result_string: String = result.iter().collect();

    println!("result_string is {}", result_string);
    result_string

    
}