use std::cmp;
use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {

    let roman_to_numerals = HashMap::from(
        [
            (String::from("I"), 1u64),
            (String::from("V"), 5),
            (String::from("X"), 10),
            (String::from("L"), 50),
            (String::from("C"), 100),
            (String::from("D"), 500),
            (String::from("M"), 1000)
        ]
    );

    let reverse_roman = roman.chars().rev().collect::<String>();
    let reverse_roman_collection_arabic: Vec<u64> = reverse_roman.chars().map(|c| roman_to_numerals.get(&c.to_string()).cloned().unwrap()).collect();

    let mut total = 0;
    let mut max = 0;

    for i in reverse_roman_collection_arabic {
        
        let calculated_max = cmp::max(i, max);
        if i >= calculated_max {
            total += i;
            max = i;
        }
        else {
            total -= i;
        }
    }

    total


}


fn main() {

    let result = roman_as_num("XLIV");
    println!("{}", result)

}