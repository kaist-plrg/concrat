use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use xml::reader::{EventReader, XmlEvent};

pub fn parse_file(file_name: &str) -> (Vec<Function>, Vec<Call>, Vec<Glob>) {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut elements = parse_document(&mut parser)
        .pop()
        .unwrap()
        .children
        .drain(..)
        .find(|e| e.name == tag("result"))
        .unwrap()
        .children;
    let functions: Vec<_> = elements
        .drain_filter(|e| e.name == tag("file"))
        .flat_map(to_file)
        .collect();
    let calls: Vec<_> = elements
        .drain_filter(|e| e.name == tag("call"))
        .map(to_call)
        .collect();
    let globs: Vec<_> = elements
        .drain_filter(|e| e.name == tag("glob"))
        .map(to_glob)
        .collect();

    (functions, calls, globs)
}

#[derive(Debug)]
pub struct Function {
    path: String,
    name: String,
    entry: String,
    ret: String,
    nodes: Vec<String>,
}

#[derive(Debug)]
pub struct Glob {
    name: String,
    analyses: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Call {
    attributes: HashMap<String, String>,
    ctxs: Vec<Ctx>,
}

#[derive(Debug)]
pub struct Ctx {
    name: String,
    analyses: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum Value {
    Data(String),
    Map(HashMap<String, Value>),
    Set(Vec<Value>),
    Analyses(HashMap<String, Value>),
}

fn to_file(element: Element) -> Vec<Function> {
    let Element {
        name,
        attributes,
        mut children,
    } = element;
    assert_eq!(name, tag("file"));
    let path = attributes.get(&"path".to_string()).unwrap();
    children
        .drain(..)
        .map(|e| to_function(e, path.clone()))
        .collect()
}

fn to_function(element: Element, path: String) -> Function {
    let Element {
        name,
        attributes,
        mut children,
    } = element;
    assert_eq!(name, tag("function"));
    let name = attributes.get(&"name".to_string()).unwrap().clone();
    let nodes: Vec<_> = children
        .drain(..)
        .map(|e| e.attributes.get(&"name".to_string()).unwrap().clone())
        .collect();
    let entry = nodes
        .iter()
        .find(|s| (*s).starts_with("fun"))
        .unwrap()
        .clone();
    let ret = nodes
        .iter()
        .find(|s| (*s).starts_with("ret"))
        .unwrap()
        .clone();
    Function {
        path,
        name,
        entry,
        ret,
        nodes,
    }
}

fn to_glob(element: Element) -> Glob {
    let Element {
        name, mut children, ..
    } = element;
    assert_eq!(name, tag("glob"));
    let mut iter = children.drain(..);
    let Element { name, children, .. } = iter.next().unwrap();
    assert_eq!(name, tag("key"));
    let name = unique_child(children).name.data();
    let analyses = iter.map(to_analysis).collect();
    Glob { name, analyses }
}

fn to_call(element: Element) -> Call {
    let Element {
        name,
        attributes,
        mut children,
    } = element;
    assert_eq!(name, tag("call"));
    let ctxs = children.drain(..).map(to_ctx).collect();
    Call { attributes, ctxs }
}

fn to_ctx(element: Element) -> Ctx {
    let Element {
        name, mut children, ..
    } = element;
    let name = name.tag();
    assert!(name == "context" || name == "path");
    let analyses = children.drain(..).map(to_analysis).collect();
    Ctx { name, analyses }
}

fn to_analysis(element: Element) -> (String, Value) {
    let Element {
        name,
        attributes,
        children,
    } = element;
    assert_eq!(name, tag("analysis"));
    let name = attributes.get("name").unwrap().clone();
    let value = to_value(unique_child(children));
    (name, value)
}

fn to_value(element: Element) -> Value {
    let Element {
        name, mut children, ..
    } = element;
    assert_eq!(name, tag("value"));

    if children.len() == 1 {
        let Element {
            name, mut children, ..
        } = unique_child(children);
        match name.tag().as_str() {
            "data" => Value::Data(unique_child(children).name.data()),
            "map" => {
                let mut map = HashMap::new();
                for (k, v) in PairIterator(&mut children.drain(..)) {
                    let Element { name, children, .. } = k;
                    assert_eq!(name, tag("key"));
                    let k = unique_child(children).name.data();
                    if let Some(v) = map.insert(k.clone(), to_value(v)) {
                        println!("duplicate key: {} {:?}", k, v);
                    }
                }
                Value::Map(map)
            }
            "set" => Value::Set(children.drain(..).map(to_value).collect()),
            s => panic!("{:?}", s),
        }
    } else {
        assert!(children.len() > 1);
        Value::Analyses(children.drain(..).map(to_analysis).collect())
    }
}

#[derive(Debug)]
struct Element {
    name: Name,
    attributes: HashMap<String, String>,
    children: Vec<Element>,
}

#[derive(Debug, PartialEq, Eq)]
enum Name {
    Tag(String),
    Data(String),
}

impl Name {
    fn tag(self) -> String {
        match self {
            Name::Tag(s) => s,
            _ => panic!(),
        }
    }

    fn data(self) -> String {
        match self {
            Name::Data(s) => s,
            _ => panic!(),
        }
    }
}

fn parse_document<T: Read>(parser: &mut EventReader<T>) -> Vec<Element> {
    let mut stack = vec![(tag(""), HashMap::new(), vec![])];
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
                    attributes: HashMap::new(),
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

fn unique_child(mut v: Vec<Element>) -> Element {
    assert_eq!(v.len(), 1);
    v.pop().unwrap()
}

fn tag(s: &str) -> Name {
    Name::Tag(s.to_string())
}

struct PairIterator<'a, T>(&'a mut dyn Iterator<Item = T>);

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0.next()?, self.0.next()?))
    }
}

fn _print_element(element: &Element, depth: usize) {
    const INDENT: &'static str = "    ";
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
        _print_element(n, depth + 1);
    }
}
