#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

fn index_of(haystack: &str, needle: char) -> Option<usize> {
    let haystack: Vec<char> = haystack.chars().collect();
    if haystack.len() > 0 && haystack[0] == needle {
        return Some(0);
    }
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn none_match() {
        // Compare Expected Value with Actual Value
        assert_eq!(None, index_of("", 'a'));
        assert_eq!(None, index_of("a", 'b'));
    }

    #[test]
    fn exact_match() {
        assert_eq!(Some(0), index_of("a", 'a'));
    }


    #[test]
    fn match_end() {
        assert_eq!(Some(2), index_of("the", 'e'));
    }



}
