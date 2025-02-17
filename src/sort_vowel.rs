// Exercise: Implement a function that sorts vowels in a string 
// while keeping consonants in their original position.
// ✅ Preserve case while sorting vowels
// ✅ Work with Unicode vowels (e.g., á, é, î, ö, ü)
// ✅ Handle large input strings efficiently

use std::{char::ToLowercase, collections::{BTreeSet, HashSet}};

pub fn sort_vowels(original_str: &str) -> String {
    // Implement sorting vowels while keeping consonants in place
    
    // initiate an empty String as result
    let mut result = String::from("");

    println!("\nOriginal String is {}", original_str);

    let valid_vowels: BTreeSet<_> = ['a', 'e', 'i', 'o', 'u', 'á', 'é', 'î', 'ö', 'ü','A', 'E', 'I', 'O', 'U', 'Á', 'É', 'Î', 'Ö', 'Ü'].into_iter().collect();

    // for (i, char) in original_str.chars().enumerate() {
    //     // if valid vowel
    //     // create two equal size vecs of vowels and indecies of 
    //     // the vowels in the original string (repeating chars stay also get added) 
    //     // println!("current index, char is (i, char) is ({}, {})", i, char);
    //     // println!("lower case of char {} is {}", char, &char.to_lowercase());
    //     if valid_lower_case_vowels.contains( &char.to_lowercase().next().unwrap()) {
    //         println!("vowel branch: (i, char) is ({}, {})", i, char);
    //         vowels_found.insert(char.clone());
    //     }
    // } 

    // automatically sorts but BTreeSet sorts with uppercase letters first
    let sorted_vowels_found: BTreeSet<_> = original_str.chars().filter(|c| valid_vowels.contains(&c.to_lowercase().next().unwrap())).collect();

    print!("sorted_vowels_found  {:?} \n", sorted_vowels_found);
    
    let mut vowels_iter = sorted_vowels_found.iter();
    // map each character of original_str to same char if constant else map to the next in the list of sorted_vowels_found
    let result = original_str.chars().map(|c| 
        if valid_vowels.contains(&c.to_lowercase().next().unwrap()) {
            *vowels_iter.next().unwrap()
        }
        else {
            c
       }
    ). collect();


    println!("result_string is {}", result);
    result
}