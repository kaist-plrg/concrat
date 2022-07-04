use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use crate::parse_xml::{Element, Name};

#[derive(Debug)]
pub struct AnalysisSummary {
    pub mutex_map: HashMap<String, Option<String>>,
    pub array_mutex_map: HashMap<String, Option<String>>,
    pub struct_mutex_map: HashMap<String, Option<String>>,
    pub node_map: HashMap<String, Vec<String>>,
    pub function_map: HashMap<String, HashMap<String, FunctionSummary>>,
}

impl AnalysisSummary {
    pub fn pretty_print(&self) {
        println!("[mutex_map]");
        for (k, v) in &self.mutex_map {
            println!("\t{}: {:?}", k, v);
        }
        println!("\n[array_mutex_map]");
        for (k, v) in &self.array_mutex_map {
            println!("\t{}: {:?}", k, v);
        }
        println!("\n[struct_mutex_map]");
        for (k, v) in &self.struct_mutex_map {
            println!("\t{}: {:?}", k, v);
        }
        println!("\n[node_map]");
        for (k, v) in &self.node_map {
            println!("\t{}: {:?}", k, v);
        }
        println!("\n[function_map]");
        for (k1, v) in &self.function_map {
            for (
                k2,
                FunctionSummary {
                    entry_mutex,
                    node_mutex,
                    ret_mutex,
                },
            ) in v
            {
                println!("\t{} in {}", k2, k1);
                println!("\t\tentry_mutex: {:?}", entry_mutex);
                println!("\t\tnode_mutex: {:?}", node_mutex);
                println!("\t\tret_mutex: {:?}", ret_mutex);
            }
        }
    }
}

#[derive(Debug)]
pub struct FunctionSummary {
    pub entry_mutex: Vec<String>,
    pub node_mutex: Vec<String>,
    pub ret_mutex: Vec<String>,
}

pub fn summarize(mut elements: Vec<Element>) -> AnalysisSummary {
    let mut elements = elements
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
    let warnings: Vec<_> = elements
        .drain_filter(|e| e.name == tag("warning"))
        .flat_map(to_warning)
        .collect();

    let mutex_map = generate_mutex_map(&globs);
    let node_map = generate_node_map(&calls);
    let function_map = generate_function_map(&functions, &node_map);
    let array_mutex_map = generate_array_mutex_map(&warnings);
    let struct_mutex_map = generate_struct_mutex_map(&warnings);

    AnalysisSummary {
        mutex_map,
        array_mutex_map,
        struct_mutex_map,
        node_map,
        function_map,
    }
}

fn generate_mutex_map(globs: &[Glob]) -> HashMap<String, Option<String>> {
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

fn generate_node_map(calls: &[Call]) -> HashMap<String, Vec<String>> {
    let k = "symb_locks".to_string();
    calls
        .iter()
        .map(|c| {
            let Call { attributes, ctxs } = c;
            let id = attributes.get(&"id".to_string()).unwrap().clone();
            let ms = ctxs
                .iter()
                .filter(|ctx| ctx.name == "path")
                .map(|ctx| {
                    ctx.analyses
                        .get(&k)
                        .unwrap()
                        .as_set()
                        .iter()
                        .cloned()
                        .map(|v| v.as_data().strip_prefix("& ").unwrap().to_string())
                        .collect::<HashSet<_>>()
                })
                .reduce(|s1, s2| s1.intersection(&s2).cloned().collect());
            (id, ms.unwrap_or_default().drain().collect())
        })
        .collect()
}

fn generate_function_map(
    funcs: &[Function],
    node_map: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, FunctionSummary>> {
    let mut map: HashMap<String, HashMap<String, FunctionSummary>> = HashMap::new();
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
        node_mutex.sort();
        node_mutex.dedup();
        let mut path = path.clone();
        path.pop();
        path.pop();
        map.entry(path).or_default().insert(
            name.clone(),
            FunctionSummary {
                entry_mutex,
                node_mutex,
                ret_mutex,
            },
        );
    }
    map
}

fn generate_array_mutex_map(warnings: &[WarningGroup]) -> HashMap<String, Option<String>> {
    warnings
        .iter()
        .filter_map(|w| {
            let WarningGroup {
                location, accesses, ..
            } = w;
            if let Some(m) = location.strip_suffix("[?]") {
                let mut mutex_set = accesses
                    .iter()
                    .map(|a| HashSet::<String>::from_iter(a.locks.iter().cloned()))
                    .reduce(|s1, s2| s1.intersection(&s2).cloned().collect())
                    .unwrap();
                assert!(mutex_set.len() <= 1);
                return Some((m.to_string(), mutex_set.drain().next()));
            }
            None
        })
        .collect()
}

fn generate_struct_mutex_map(warnings: &[WarningGroup]) -> HashMap<String, Option<String>> {
    let mut map: HashMap<String, Vec<Option<String>>> = HashMap::new();
    for WarningGroup {
        location, accesses, ..
    } in warnings
    {
        if let Some(i) = location.rfind('.') {
            let field = location[i + 1..].to_string();
            let mutex_set = accesses
                .iter()
                .map(|a| HashSet::<String>::from_iter(a.locks.iter().cloned()))
                .reduce(|s1, s2| s1.intersection(&s2).cloned().collect())
                .unwrap();
            let mut mutex_vec: Vec<_> = mutex_set
                .iter()
                .filter_map(|s| s.strip_prefix("*.").map(|s| s.to_string()))
                .collect();
            assert!(mutex_vec.len() <= 1);
            map.entry(field)
                .or_default()
                .push(mutex_vec.drain(..).next());
        }
    }
    map.drain()
        .map(|(m, mut v)| {
            v.sort();
            v.dedup();
            assert!(v.len() == 1);
            (m, v.drain(..).next().unwrap())
        })
        .collect()
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
pub struct WarningGroup {
    pub location: String,
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub safe: bool,
    pub accesses: Vec<Access>,
}

#[derive(Debug, Clone)]
pub struct Access {
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub read: bool,
    pub region: String,
    pub locks: Vec<String>,
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

fn to_warning(element: Element) -> Option<WarningGroup> {
    let Element { name, children, .. } = element;
    assert_eq!(name, tag("warning"));
    let Element {
        name,
        attributes,
        mut children,
        ..
    } = unique_child(children);
    match name.tag().as_str() {
        "group" => {
            let info = attributes.get(&"name".to_string()).unwrap();
            let info = &info[16..];

            let (location, info) = find_and_split(info, '@');
            let (path, info) = find_and_split(info, ':');
            let (line, info) = find_and_split(info, ':');
            let line = line.parse().unwrap();
            let (column, info) = find_and_split(info, ' ');
            let column = column.parse().unwrap();
            let safe = info == "(safe)";
            let accesses = children.drain(..).map(to_access).collect();
            Some(WarningGroup {
                location,
                path,
                line,
                column,
                safe,
                accesses,
            })
        }
        "text" => None,
        _ => unreachable!(),
    }
}

fn to_access(element: Element) -> Access {
    let Element {
        name,
        attributes,
        children,
    } = element;
    assert_eq!(name, tag("text"));
    let path = attributes.get(&"file".to_string()).unwrap().clone();
    let line = attributes
        .get(&"line".to_string())
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let column = attributes
        .get(&"column".to_string())
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let data = unique_child(children).name.data();
    let (read, data) = find_and_split(&data, ' ');
    let read = read == "read";
    let (_, data) = find_and_split(data, ':');
    let (region, data) = find_and_split(data, '}');
    let (_, data) = find_and_split(data, '{');
    let mut locks: Vec<_> = data
        .split(", ")
        .filter_map(|s| {
            s.strip_prefix("lock:")
                .or_else(|| s.strip_prefix("i-lock:"))
                .or_else(|| s.strip_prefix("p-lock:"))
                .map(|s| s.split('[').next().unwrap().to_string())
        })
        .collect();
    locks.sort();
    locks.dedup();

    Access {
        path,
        line,
        column,
        read,
        region,
        locks,
    }
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
            "data" => {
                let child = unique_child(children);
                match &child.name {
                    Name::Tag(t) => {
                        assert_eq!(t, "value");
                        to_value(child)
                    }
                    Name::Data(d) => Value::Data(d.clone()),
                }
            }
            "map" => {
                let mut map = HashMap::new();
                for (k, v) in PairIterator(&mut children.drain(..)) {
                    let Element { name, children, .. } = k;
                    assert_eq!(name, tag("key"));
                    let k = unique_child(children).name.data();
                    map.insert(k.clone(), to_value(v));
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
    fn as_map(&self) -> &HashMap<String, Value> {
        if let Value::Map(m) = self {
            m
        } else {
            panic!()
        }
    }

    fn try_as_set(&self) -> Option<&Vec<Value>> {
        if let Value::Set(s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn as_set(&self) -> &Vec<Value> {
        if let Value::Set(s) = self {
            s
        } else {
            panic!()
        }
    }

    fn as_data(&self) -> &String {
        if let Value::Data(d) = self {
            d
        } else {
            panic!()
        }
    }
}

fn unique_child(mut v: Vec<Element>) -> Element {
    assert_eq!(v.len(), 1);
    v.pop().unwrap()
}

fn tag(s: &str) -> Name {
    Name::Tag(s.to_string())
}

fn find_and_split(s: &str, c: char) -> (String, &str) {
    let i = s.find(c).unwrap();
    (s[..i].to_string(), &s[i + 1..])
}

struct PairIterator<'a, T>(&'a mut dyn Iterator<Item = T>);

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0.next()?, self.0.next()?))
    }
}
