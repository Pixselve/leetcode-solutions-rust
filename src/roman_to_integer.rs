struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let iterator = s.chars();
        let mut count = 0;
        let mut previous_char: Option<char> = None;

        for character in iterator {
            match (character, previous_char) {
                ('I', _) => count += 1,
                ('V', Some('I')) => count += 4 - 1,
                ('V', _) => count += 5,
                ('X', Some('I')) => count += 9 - 1,
                ('X', _) => count += 10,
                ('L', Some('X')) => count += 40 - 10,
                ('L', _) => count += 50,
                ('C', Some('X')) => count += 90 - 10,
                ('C', _) => count += 100,
                ('D', Some('C')) => count += 400 - 100,
                ('D', _) => count += 500,
                ('M', Some('C')) => count += 900 - 100,
                ('M', _) => count += 1000,

                _ => {}
            }
            previous_char = Some(character);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::roman_to_int(String::from("I")), 1);
        assert_eq!(Solution::roman_to_int(String::from("II")), 2);
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("V")), 5);
        assert_eq!(Solution::roman_to_int(String::from("VI")), 6);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
