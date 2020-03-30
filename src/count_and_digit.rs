/*

Take an integer n (n >= 0) and a digit d (0 <= d <= 9) as an integer. Square
 all numbers k (0 <= k <= n) between 0 and n. Count the numbers of digits d
 used in the writing of all the k**2. Call nb_dig (or nbDig or ...)
  
 The function taking n and d as parameters and returning this count.

#Examples:

n = 10, d = 1, the k*k are 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
We are using the digit 1 in 1, 16, 81, 100. The total count is then 4.

nb_dig(25, 1):
the numbers of interest are
1, 4, 9, 10, 11, 12, 13, 14, 19, 21 which squared are 1, 16, 81, 100, 121, 144, 169, 196, 361, 441
so there are 11 digits `1` for the squares of numbers between 0 and 25.

##################
Solution:

fn nb_dig(n: i32, d: i32) -> i32 {
    let ex = d.to_string();
    (0..=n).map(|v| (v*v).to_string()).fold(0, |acc, v| acc + v.matches(&ex).count() as i32)
}

fn nb_dig(n: i32, d: i32) -> i32 {
    (0..=n).fold(0, |r, x| r + (x * x).to_string().chars().filter(|c| c.to_digit(10).unwrap() as i32 == d).count() as i32)
}

*/

fn nb_dig(n: i32, d: i32) -> i32 {
    // your code
    let mut counter = 0;
    for x in 0..n+1{
        counter+= number_occurrences(x*x, d);
        
    }
    counter
}

fn number_occurrences(number: i32, d:i32) -> i32 {
    let mut occurrences = 0;
    let number_string : String = format!("{}", number);
    let d_string :String = format!("{}", d);
    number_string.matches(&d_string).count() as i32
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
        
    }
}