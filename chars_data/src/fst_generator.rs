use std::collections::btree_map;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

fn build_stopwords() -> BTreeSet<&'static str> {
    BTreeSet::from_iter(
        vec![
            "with",
            "sign",
            "small",
            "letter",
            "digit",
            "for",
            "symbol",
            "<control>",
        ]
        .into_iter(),
    )
}

#[test]
fn test_stopwords() {
    let sw = build_stopwords();
    assert!(sw.contains("with"));
}

fn add_component(result: &mut Vec<String>, component: &str) {
    lazy_static! {
        static ref STOPS: BTreeSet<&'static str> = build_stopwords();
    }
    let component = component.to_lowercase();
    let component = component.trim_end_matches(',');

    if STOPS.contains(component) || component.len() == 1 {
        return;
    }
    result.push(component.to_owned());

    if !component.contains('-') {
        return;
    }
    for parts in component.split('-') {
        add_component(result, parts);
    }
}

fn name_components(name: &str) -> Vec<String> {
    let mut result = vec![];
    result.push(name.to_lowercase());
    for component in name.split_whitespace() {
        add_component(&mut result, component);
    }
    result
}

#[test]
fn test_name_components() {
    assert_eq!(
        name_components("D WITH CURL, LATIN SMALL LETTER"),
        Vec::from_iter(
            vec!["d with curl, latin small letter", "curl", "latin"]
                .into_iter()
                .map(String::from)
        )
    );
    assert_eq!(
        name_components("Equals Sign"),
        Vec::from_iter(vec!["equals sign", "equals"].into_iter().map(String::from))
    );
    assert_eq!(
        name_components("UPSIDE-DOWN FACE"),
        Vec::from_iter(
            vec!["upside-down face", "upside-down", "upside", "down", "face"]
                .into_iter()
                .map(String::from)
        )
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct Names {
    map: BTreeMap<String, BTreeSet<char>>,
}

impl Names {
    pub fn new() -> Names {
        Names {
            map: BTreeMap::new(),
        }
    }

    ///Insert a character names' components into btree map, minus stopwords.
    pub fn insert(&mut self, names: Vec<String>, ch: char) {
        for name in names {
            for component in name_components(name.as_str()) {
                self.map
                    .entry(component)
                    .or_insert_with(BTreeSet::new)
                    .insert(ch);
            }
        }
    }

    pub fn iter(&self) -> btree_map::Iter<String, BTreeSet<char>> {
        self.map.iter()
    }
}
