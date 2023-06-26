struct CustomIter {
    v: Vec<String>,
    counter: usize,
}
impl CustomIter {
    fn new() -> Self {
        CustomIter {
            v: Vec::new(),
            counter: 0,
        }
    }

    fn add(&mut self, val: String) {
        self.v.push(val)
    }
}

impl Iterator for CustomIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v.get(self.counter) {
            Some(s) => {
                self.counter += 1;
                Some(s.to_owned())
            }
            None => None,
        }
    }
}


pub fn test_custom_iterator() {
    let mut cis = CustomIter::new();

    cis.add("Amirreza".to_string());
    cis.add("Parsa".to_string());
    cis.add("Yas".to_string());

    for fruit in cis {
        println!("{}", fruit);
    }
}

