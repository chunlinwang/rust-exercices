#[derive(Debug)]
pub struct Split {
    remainder: &'static str,
    delimiter: char,
}

impl Split {
    pub fn new(input: &'static str, delimiter: char) -> Self {
        Self {
            remainder: input,
            delimiter,
        }
    }
}

impl Iterator for Split {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + 1)..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[cfg(test)]
mod tests_char {
    use super::*;

    #[test]
    fn nominal() {
        let input = "a,b,c";
        let expected = input.split(',').collect::<Vec<_>>();

        let result = Split::new(input, ',').collect::<Vec<_>>();

        assert_eq!(result, expected);
    }

    // You can add more tests.
}

fn main() {
    let input = "a,b,c";
    let expected = input.split(',').collect::<Vec<_>>();

    let result = Split::new(input, ',').collect::<Vec<_>>();

    assert_eq!(result, expected);
}
