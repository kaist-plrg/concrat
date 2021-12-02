use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use xml::reader::{EventReader, XmlEvent};

pub fn generate_mutex_map(globs: &[Glob]) -> HashMap<String, Option<String>> {
    let k1 = "mutex".to_string();
    let k2 = "readwrite".to_string();

    globs
        .iter()
        .map(|g| {
            let Glob { name, analyses } = g;

            let mutex_map = analyses.get(&k1).unwrap().as_map();

            let v = vec![];
            let mut readwrite_set = mutex_map
                .get(&k2)
                .unwrap()
                .try_as_set()
                .unwrap_or(&v)
                .iter()
                .map(|v| v.as_data().clone())
                .collect::<Vec<_>>();

            assert!(readwrite_set.len() <= 1);

            (name.clone(), readwrite_set.pop())
        })
        .collect()
}

pub fn generate_node_map(calls: &[Call]) -> HashMap<String, Vec<String>> {
    let k1 = "mutex".to_string();
    let k2 = "Normal Lvals".to_string();
    calls
        .iter()
        .map(|c| {
            let Call { attributes, ctxs } = c;
            let id = attributes.get(&"id".to_string()).unwrap().clone();
            let mut ms = ctxs
                .iter()
                .filter(|ctx| ctx.name == "path")
                .flat_map(|ctx| ctx.analyses.get(&k1).unwrap().as_set().clone())
                .map(|v| v.as_map().get(&k2).unwrap().as_data().clone())
                .collect::<Vec<_>>();
            ms.dedup();
            (id, ms)
        })
        .collect()
}

pub fn generate_function_map(
    funcs: &[Function],
    node_map: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, (Vec<String>, Vec<String>, Vec<String>)>> {
    let mut map: HashMap<String, HashMap<String, (Vec<String>, Vec<String>, Vec<String>)>> =
        HashMap::new();
    for f in funcs {
        let Function {
            path,
            name,
            entry,
            ret,
            nodes,
        } = f;
        let entry_mutex = node_map.get(entry).unwrap().clone();
        let ret_mutex = node_map.get(ret).unwrap().clone();
        let mut node_mutex: Vec<_> = nodes
            .iter()
            .flat_map(|n| node_map.get(n).unwrap().clone())
            .collect();
        node_mutex.dedup();
        let mut path = path.clone();
        path.pop();
        path.pop();
        map.entry(path)
            .or_default()
            .insert(name.clone(), (entry_mutex, node_mutex, ret_mutex));
    }
    map
}

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

#[derive(Debug, Clone)]
pub struct Function {
    pub path: String,
    pub name: String,
    pub entry: String,
    pub ret: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Glob {
    pub name: String,
    pub analyses: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub attributes: HashMap<String, String>,
    pub ctxs: Vec<Ctx>,
}

#[derive(Debug, Clone)]
pub struct Ctx {
    pub name: String,
    pub analyses: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
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
    let mut nodes: Vec<_> = children
        .drain(..)
        .map(|e| e.attributes.get(&"name".to_string()).unwrap().clone())
        .collect();

    let mut entry: Vec<_> = nodes.drain_filter(|s| (*s).starts_with("fun")).collect();
    assert_eq!(entry.len(), 1);
    let entry = entry.pop().unwrap();

    let mut ret: Vec<_> = nodes.drain_filter(|s| (*s).starts_with("ret")).collect();
    assert_eq!(ret.len(), 1);
    let ret = ret.pop().unwrap();

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

impl Value {
    pub fn as_map(&self) -> &HashMap<String, Value> {
        if let Value::Map(m) = self {
            m
        } else {
            panic!()
        }
    }

    pub fn try_as_set(&self) -> Option<&Vec<Value>> {
        if let Value::Set(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_set(&self) -> &Vec<Value> {
        if let Value::Set(s) = self {
            s
        } else {
            panic!()
        }
    }

    pub fn as_data(&self) -> &String {
        if let Value::Data(d) = self {
            d
        } else {
            panic!()
        }
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
