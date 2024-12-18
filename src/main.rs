use std::cmp;
use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {
    // Thanks a lot! -> http://blog.functionalfun.net/2009/01/project-euler-89-converting-to-and-from.html
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

fn num_as_roman(decimal_number: u16) -> String{
    // Thanks! -> https://stackoverflow.com/questions/9083037/convert-a-number-into-a-roman-numeral-in-javascript
    // {M:1000,CM:900,D:500,CD:400,C:100,XC:90,L:50,XL:40,X:10,IX:9,V:5,IV:4,I:1}
    let romans_to_numerals = HashMap::from(
        [
            ("M", 1000u16),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1)
        ]
    );

    let mut _decimal_iterator = decimal_number;
    let mut roman_number= String::from("");
    for i in vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"] {
        while _decimal_iterator >= romans_to_numerals[i] {
            roman_number += i;
            _decimal_iterator -= romans_to_numerals[i];
        }
    }

    roman_number
     


}

fn main() {

    let number_devil = 666;
    let roman_heaven = "XLIV";

    println!("Number from heaven XLIV is {}", roman_as_num(roman_heaven));
    println!("Number from hell 666 is roman {}", num_as_roman(number_devil))
}

#[cfg(test)]
mod tests_num_as_roman {
    use super::*;

    #[test]
    fn case_1() {
        let result = num_as_roman(44);
        assert_eq!(result, "XLIV");
    }

    #[test]
    fn case_2() {
        let result = num_as_roman(666);
        assert_eq!(result, "DCLXVI");
    }

    #[test]
    fn case_3() {
        let result = num_as_roman(99);
        assert_eq!(result, "XCIX");
    }

    #[test]
    fn case_4() {
        let result = num_as_roman(55);
        assert_eq!(result, "LV");
    }
}

#[cfg(test)]
mod tests_roman_as_num {
    use super::*;

    #[test]
    fn case_1() {
        let result = roman_as_num("V");
        assert_eq!(result, 5);
    }

    #[test]
    fn case_2() {
        let result = roman_as_num("DCLXVI");
        assert_eq!(result, 666);
    }

    #[test]
    fn case_3() {
        let result = roman_as_num("XCIX");
        assert_eq!(result, 99);
    }

    #[test]
    fn case_4() {
        let result = roman_as_num("LV");
        assert_eq!(result, 55);

}
}