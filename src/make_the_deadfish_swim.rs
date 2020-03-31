/*

Write a simple parser that will parse and run Deadfish.

Deadfish has 4 commands, each 1 character long:

i increments the value (initially 0)
d decrements the value
s squares the value
o outputs the value into the return array
Invalid characters should be ignored.

parse("iiisdoso") => [ 8, 64 ]

fn parse(code: &str) -> Vec<i32> {
    let mut output = Vec::new();
    code.chars().fold(0, |val, cmd| {
        match cmd {
            'i' => val + 1,
            'd' => val - 1,
            's' => val * val,
            'o' => {
                output.push(val);
                val
            },
            _ => val,
        }
    });
    output
}

*/

fn parse(code: &str) -> Vec<i32> {
    let mut output = Vec::new();
    let mut counter : i32 = 0;
    for character in code.chars() {
            match character {
                'i' => counter += 1,
                'd' => counter -= 1,
                's' => counter *= counter,
                'o' => output.push(counter),
                _   => ()
            }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}