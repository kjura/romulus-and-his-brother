use std::cmp;
use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {

    let roman_to_numerals = HashMap::from(
        [
            ('I', 1u64),
            ('V', 5),
            ('X', 10), 
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ]
    );

    let reverse_roman_collection_arabic: Vec<u64> = roman.chars().rev().map(|c| roman_to_numerals.get(&c).cloned().unwrap()).collect();

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

    // [5, 1, 50, 10] --> 0 + 5 - 1 + 50 - 10 = 44
    // acc = 0
    // if max( if max(0, 5) >= 5 { acc + 5 } else { acc - 5 } ) >= 1 { acc + 1 } else { acc - 1 }

}