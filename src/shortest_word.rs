/*

Simple, given a string of words, return the length of the shortest word(s).

String will never be empty and you do not need to account for different data types.



##########
Best Solution
fn find_short(s: &str) -> usize {
  s.split_whitespace().map(str::len).min().unwrap()
}

*/
fn find_short(s: &str) -> u32 {
    //your code here
    let vec : Vec<&str> = s.split_ascii_whitespace().collect();
    let mut minimum = u32::max_value();
    for elem in vec {
        let length = elem.len() as u32;
        if length < minimum {
            minimum = length;
        }
    }
    minimum
}

#[test]
fn returns_expected() {
  assert_eq!(find_short("bitcoin take over the world maybe who knows perhaps"), 3);
  assert_eq!(find_short("turns out random test cases are easier than writing out basic ones"), 3);
  assert_eq!(find_short("lets talk about javascript the best language"), 3);
  assert_eq!(find_short("i want to travel the world writing code one day"), 1);
  assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
}