use std::collections::{HashMap, HashSet};

use crate::parse_xml::{Element, Name};

#[derive(Debug)]
pub struct AnalysisSummary {
    pub mutex_map: HashMap<String, String>,
    pub array_mutex_map: HashMap<String, String>,
    pub struct_mutex_map: HashMap<String, HashMap<String, String>>,
    pub node_map: HashMap<String, Vec<String>>,
    pub function_map: HashMap<String, FunctionSummary>,
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
        for (
            k,
            FunctionSummary {
                entry_mutex,
                node_mutex,
                ret_mutex,
            },
        ) in &self.function_map
        {
            println!("\t{}", k);
            println!("\t\tentry_mutex: {:?}", entry_mutex);
            println!("\t\tnode_mutex: {:?}", node_mutex);
            println!("\t\tret_mutex: {:?}", ret_mutex);
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
    let warnings: Vec<_> = elements
        .drain_filter(|e| e.name == tag("warning"))
        .flat_map(to_warning)
        .collect();

    let (mutex_map, array_mutex_map, struct_mutex_map) = generate_mutex_maps(&warnings);
    let node_map = generate_node_map(&calls);
    let function_map = generate_function_map(&functions, &node_map);

    AnalysisSummary {
        mutex_map,
        array_mutex_map,
        struct_mutex_map,
        node_map,
        function_map,
    }
}

fn generate_mutex_maps(
    warnings: &[WarningGroup],
) -> (
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, HashMap<String, String>>,
) {
    let mut global_mutex_map = HashMap::new();
    let mut array_mutex_map = HashMap::new();
    let mut struct_mutex_map = HashMap::new();
    for WarningGroup {
        name,
        typ,
        protections,
    } in warnings
    {
        let mut plocks: Vec<_> = protections
            .iter()
            .filter(|p| matches!(p, Protection::PLock(_)))
            .collect();
        if let Some(Protection::PLock(plock)) = plocks.pop() {
            assert_eq!(plocks.len(), 0);
            let (typ, field) = find_and_split(typ, '.');
            let field = if let Some(i) = field.find('.') {
                &field[..i]
            } else {
                field
            };
            struct_mutex_map
                .entry(typ)
                .or_insert_with(HashMap::new)
                .insert(field.to_string(), plock.to_string());
            continue;
        }

        let mut ilocks: Vec<_> = protections
            .iter()
            .filter(|p| matches!(p, Protection::ILock(_)))
            .collect();
        if let Some(Protection::ILock(ilock)) = ilocks.pop() {
            assert_eq!(ilocks.len(), 0);
            array_mutex_map.insert(name.to_string(), ilock.to_string());
            continue;
        }

        if let Some(Protection::Lock(lock)) = protections.last() {
            assert_eq!(protections.len(), 1);
            global_mutex_map.insert(name.to_string(), lock.to_string());
            continue;
        }

        unreachable!();
    }
    (global_mutex_map, array_mutex_map, struct_mutex_map)
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
) -> HashMap<String, FunctionSummary> {
    let mut map: HashMap<String, FunctionSummary> = HashMap::new();
    for f in funcs {
        let Function {
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
        map.insert(
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

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub entry: String,
    pub ret: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WarningGroup {
    pub name: String,
    pub typ: String,
    pub protections: Vec<Protection>,
}

#[derive(Debug, Clone)]
pub enum Protection {
    Lock(String),
    ILock(String),
    PLock(String),
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
        name, mut children, ..
    } = element;
    assert_eq!(name, tag("file"));
    children.drain(..).flat_map(to_function).collect()
}

fn to_function(element: Element) -> Option<Function> {
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

    if nodes.is_empty() {
        return None;
    }

    let mut entry: Vec<_> = nodes.drain_filter(|s| (*s).starts_with("fun")).collect();
    assert_eq!(entry.len(), 1);
    let entry = entry.pop().unwrap();

    let mut ret: Vec<_> = nodes.drain_filter(|s| (*s).starts_with("ret")).collect();
    assert_eq!(ret.len(), 1);
    let ret = ret.pop().unwrap();

    Some(Function {
        name,
        entry,
        ret,
        nodes,
    })
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
            let name = attributes.get("name").unwrap();
            let (name, typ) = find_and_split(name, ';');
            let protections = children.drain(..).map(to_protection).collect();
            Some(WarningGroup {
                name,
                typ: typ.to_string(),
                protections,
            })
        }
        "text" => None,
        _ => unreachable!(),
    }
}

fn to_protection(element: Element) -> Protection {
    let Element { name, children, .. } = element;
    assert_eq!(name, tag("text"));
    let data = unique_child(children).name.data();
    if let Some(s) = data.strip_prefix("lock:") {
        return Protection::Lock(s.to_string());
    }
    if let Some(s) = data.strip_prefix("i-lock:") {
        let (s, _) = find_and_split(s, '[');
        return Protection::ILock(s);
    }
    if let Some(s) = data.strip_prefix("p-lock:") {
        let (_, s) = find_and_split(s, '.');
        return Protection::PLock(s.to_string());
    }
    unreachable!()
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
