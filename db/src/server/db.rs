use std::collections::HashMap;

// 定义数据结构
pub struct Record {
    id: i32,
    name: String,
    age: i32,
}

// 定义B+树节点
pub struct Node {
    keys: Vec<i32>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node { keys: Vec::new(), children: Vec::new() }
    }
}

// 定义B+树
pub struct BPlusTree {
    root: Box<Node>,
    m: usize,
}

impl BPlusTree {
    fn new(m: usize) -> BPlusTree {
        BPlusTree { root: Box::new(Node::new()), m }
    }

    // 插入数据
    fn insert(&mut self, key: i32, record: Record) {
        if self.root.keys.len() == 0 {
            self.root.keys.push(key);
            self.root.children.push(Box::new(Node::new()));
            self.root.children.push(Box::new(Node::new()));
            return;
        }

        let mut cur_node = &mut self.root;
        let mut parent_node = None;
        let mut index = 0;

        loop {
            index = match cur_node.keys.binary_search(&key) {
                Ok(i) => i,
                Err(i) => i,
            };

            if index < cur_node.keys.len() && cur_node.keys[index] == key {
                // Key already exists, update the record
                // ...
                return;
                //return Some(cur_node.records[index].replace(record));
            }

            if cur_node.children.is_empty() {
                // Leaf node, insert the key
                cur_node.keys.insert(index, key);
                cur_node.children.insert(index, Box::new(Node::new()));
                break;
            } else {
                // Internal node, go to the child
                parent_node = Some(cur_node);
                cur_node = &mut cur_node.children[index];
            }
        }

        // Split the node if it's full
        if cur_node.keys.len() > self.m {
            let new_node = self.split_node(cur_node);
            parent_node.unwrap().children.insert(index + 1, Box::new(new_node));
        }
    }

    // Split a node into two
    fn split_node(&self, node: &mut Node) -> Node {
        let mid = node.keys.len() / 2;
        let mut new_node = Node::new();

        new_node.keys = node.keys.split_off(mid);
        new_node.children = node.children.split_off(mid);

        new_node
    }
}

// 定义数据库
pub struct Database {
    tables: HashMap<String, BPlusTree>,
}

impl Database {
    fn new() -> Database {
        Database { tables: HashMap::new() }
    }

    // 创建表
    fn create_table(&mut self, table_name: String, m: usize) {
        self.tables.insert(table_name, BPlusTree::new(m));
    }

    // 插入数据
    fn insert(&mut self, table_name: String, record: Record) {
        if !self.tables.contains_key(&table_name) {
            self.create_table(table_name.clone(), 4); // Here 4 is the maximum number of children for a node
        }
        self.tables.get_mut(&table_name).unwrap().insert(record.id, record);
    }
}

// 使用示例
fn main() {
    let mut db = Database::new();
    let record = Record { id: 1, name: "Alice".to_string(), age: 20 };
    db.insert("users".to_string(), record);
}
