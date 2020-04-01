/*

Don't give me five!
In this kata you get the start number and the end number of a region and
should return the count of all numbers except numbers with a 5 in it.
 
The start and the end number are both inclusive!

Examples:

1,9 -> 1,2,3,4,6,7,8,9 -> Result 8
4,17 -> 4,6,7,8,9,10,11,12,13,14,16,17 -> Result 12
The result may contain fives. ;-)
The start number will always be smaller than the end number.
Both numbers can be also negative!

I'm very curious for your solutions and the way you solve it.
Maybe someone of you will find an easy pure mathematics solution.

Have fun coding it and please don't forget to vote and rank this kata! :-)

I have also created other katas. Take a look if you enjoyed this kata!

#################
Solution

fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..end+1)
      .filter(|x| !x.to_string().contains('5'))
      .count() as isize
}

*/

// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

fn dont_give_me_five(start: isize, end: isize) -> isize {
    let mut counter = 0;
    for elem in start..=end {
        let numberElem = format!("{}", elem);
        if !numberElem.contains("5") {
            counter+=1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
use super::dont_give_me_five;

#[test]
fn returns_expected() {
    assert_eq!(dont_give_me_five(1, 9), 8);
    assert_eq!(dont_give_me_five(4, 17), 12);
}
}