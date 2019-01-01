use minidom::Element;

pub trait DomUtils {
    fn descend(&self) -> Descend;
    fn expect_parse_attr<T: std::str::FromStr>(&self, name: &str) -> T;
    fn expect_child(&self, name: &str) -> &Self;
    fn expect_parse_child<T: std::str::FromStr>(&self, name: &str) -> T;
}

impl DomUtils for Element {
    fn descend(&self) -> Descend {
        Descend {
            root: self,
            results: None,
            index: 0,
        }
    }

    fn expect_parse_attr<T: std::str::FromStr>(&self, name: &str) -> T {
        self.attr(name).expect(name).parse::<T>().ok().unwrap()
    }

    fn expect_child(&self, name: &str) -> &Self {
        let matches: Vec<&Self> = self.children().filter(|el| el.name() == name).collect();
        assert!(matches.len() == 1);
        matches[0]
    }

    fn expect_parse_child<T: std::str::FromStr>(&self, name: &str) -> T {
        let matches: Vec<&Self> = self.children().filter(|el| el.name() == name).collect();
        assert!(matches.len() == 1);
        matches[0].text().parse::<T>().ok().unwrap()
    }
}

pub struct Descend<'a> {
    root: &'a Element,
    results: Option<Vec<&'a Element>>,
    index: usize,
}

impl<'a> std::iter::FusedIterator for Descend<'a> {}
impl<'a> std::iter::Iterator for Descend<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<&'a Element> {
        if self.index == 0 {
            self.index = 1;
            Some(self.root)
        } else {
            if self.results.is_none() {
                self.results = Some(
                    self.root
                        .children()
                        .map(|c| c.descend())
                        .flatten()
                        .collect(),
                );
            }
            if let Some(results) = &mut self.results {
                if self.index >= results.len() {
                    if !results.is_empty() {
                        results.clear();
                        results.shrink_to_fit();
                    }
                    None
                } else {
                    let result = results[self.index];
                    self.index += 1;
                    Some(result)
                }
            } else {
                unreachable!("results cannot still be None")
            }
        }
    }
}
