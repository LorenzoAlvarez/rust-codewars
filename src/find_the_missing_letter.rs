/*

#Find the missing letter 
user5036852

Write a method that takes an array of consecutive (increasing) letters as input
 and that returns the missing letter in the array.

You will always get an valid array. And it will be always exactly one letter be
 missing. The length of the array will always be at least 2.
The array will always contain letters in only one case.

Example:

['a','b','c','d','f'] -> 'e' ['O','Q','R','S'] -> 'P'

["a","b","c","d","f"] -> "e"
["O","Q","R","S"] -> "P"
(Use the English alphabet with 26 letters!)

Have fun coding it and please don't forget to vote and rank this kata! :-)

I have also created other katas. Take a look if you enjoyed this kata!

*/

fn find_missing_letter(chars: &[char]) -> char {
    let mut expected : u8 = *chars.first().unwrap() as u8;
    println!("{}", expected);
    for c in chars{
        println!("Expected: {} : Output {}",expected, *c as u8);
        if (*c as u8) - expected == 0 {
            expected += 1;
        }
        else{
            return expected as char;
        }
         
    }
    expected as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}