use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub struct Element {
    pub name: Name,
    pub attributes: BTreeMap<String, String>,
    pub children: Vec<Element>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Name {
    Tag(String),
    Data(String),
}

impl Name {
    pub fn tag(self) -> String {
        match self {
            Name::Tag(s) => s,
            _ => panic!(),
        }
    }

    pub fn data(self) -> String {
        match self {
            Name::Data(s) => s,
            _ => panic!(),
        }
    }
}

pub fn parse_file(file_name: &str) -> Vec<Element> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);
    let mut parser = EventReader::new(file);
    parse_document(&mut parser)
}

fn parse_document<T: Read>(parser: &mut EventReader<T>) -> Vec<Element> {
    let mut stack = vec![(Name::Tag(String::new()), BTreeMap::new(), vec![])];
    loop {
        match parser.next().unwrap() {
            XmlEvent::StartElement {
                name,
                mut attributes,
                ..
            } => {
                let attributes = attributes
                    .drain(..)
                    .map(|a| (a.name.local_name, a.value))
                    .collect();
                stack.push((Name::Tag(name.local_name), attributes, vec![]));
            }
            XmlEvent::EndElement { name } => {
                let (tag, attributes, children) = stack.pop().unwrap();
                assert_eq!(tag, Name::Tag(name.local_name));
                let elem = Element {
                    name: tag,
                    attributes,
                    children,
                };
                stack.last_mut().unwrap().2.push(elem);
            }
            XmlEvent::Characters(s) => {
                let elem = Element {
                    name: Name::Data(s.trim().to_string()),
                    attributes: BTreeMap::new(),
                    children: vec![],
                };
                stack.last_mut().unwrap().2.push(elem);
            }
            XmlEvent::EndDocument => break,
            XmlEvent::StartDocument { .. } | XmlEvent::Whitespace(_) => (),
            e => panic!("{:?}", e),
        }
    }
    stack.pop().unwrap().2
}

#[allow(dead_code)]
fn print_element(element: &Element, depth: usize) {
    const INDENT: &str = "    ";
    let i = (0..depth)
        .map(|_| INDENT)
        .fold(String::with_capacity(depth * INDENT.len()), |r, s| r + s);
    let Element {
        name,
        attributes,
        children,
    } = element;
    println!("{}{:?} {:?}", i, name, attributes);
    for n in children {
        print_element(n, depth + 1);
    }
}
