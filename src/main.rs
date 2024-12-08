use core::num;
use std::cmp;
use std::{collections::HashMap, error, hash::Hash};

fn roman_as_num(roman: &str) -> u64 {
    todo!("transform the string roman numeral into an integer")
}



fn get_digits(number: i32) -> Vec<i32> {

    let mut a:i32 = number;
    let b:i32 = 10;
    let mut container: Vec<i32> = Vec::new();

    for _ in 0..a.to_string().len() {

        container.push(a.rem_euclid(b));
        a /= 10;

    }

    container

}

fn main() {


    let roman_to_numerals = HashMap::from(
        [
            (String::from("I"), 1),
            (String::from("V"), 5),
            (String::from("X"), 10),
            (String::from("L"), 50),
            (String::from("C"), 100),
            (String::from("D"), 500),
            (String::from("M"), 1000)
        ]
    );

    let numerals_to_roman: HashMap<i32, String> = roman_to_numerals.iter().map(| (k, v) | (*v, k.clone())).collect();
    // 3999 will be max for our purposes


    let liczba = "CCCXXIV";

    let odwrocona_liczba = liczba.chars().rev().collect::<String>();
    // potrzebuję iterować po odwróconym stringu na char'ach, i podmieniać je z hashmapy na liczby
    // e.g XIV staje sie [V, I, X] => [5, 1, 10] 


    let kolekcja: Vec<i32> = odwrocona_liczba.chars().map(|c| roman_to_numerals.get(&c.to_string()).cloned().unwrap()).collect();


    let mut total = 0;
    let mut max = 0;

    for i in kolekcja {
        
        let calculated_max = cmp::max(i, max);
        if i >= calculated_max {
            total += i;
            max = i;
        }
        else {
            total -= i;
        }
    }

    println!("{}", total)


    

    // println!("{:#?}", odwrocona_liczba);

    // println!("{:#?}", kolekcja)

    // println!("{:#?}", dupa)

    // 5 1 10 ==> 
    // max(0, 5)=5 => 5 >= 5 => 0 + 5 = 5
    // max(5, 1)= 5 => 1 < 5 => 5 - 1

}