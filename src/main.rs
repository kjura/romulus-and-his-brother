use std::{collections::HashMap, hash::Hash};

fn roman_as_num(roman: &str) -> u64 {
    todo!("transform the string roman numeral into an integer")
}



fn get_digits(number: i32) -> Vec<i32> {

    let mut a:i32 = 1460;
    let b:i32 = 10;
    let mut container: Vec<i32> = Vec::new();

    for _ in 0..a.to_string().len() {

        container.push(a.rem_euclid(b));
        a /= 10;

    }

    container

}

fn main() {


    // let roman_to_numerals = HashMap::from(
    //     [
    //         ("I", 1),
    //         ("V", 5),
    //         ("X", 10),
    //         ("L", 50),
    //         ("C", 100),
    //         ("D", 500),
    //         ("M", 1000)
    //     ]
    // );

    // let numerals_to_roman: HashMap<i32, &str> = roman_to_numerals.iter().map(| (k, v) | (*v, *k)).collect();


    let liczba = 1460;
    let container = get_digits(liczba);
    let mut values: Vec<i32> = Vec::new();


    let mut multiplier = 1;
    for i in &container {
        values.push((i * multiplier));
        multiplier *= 10;
    }

    println!("{:#?}", container);
    println!("{:#?}", values);



}