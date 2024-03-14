use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    // 可以根据需要添加更多类型
}

#[derive(Debug, Clone)]
pub struct BPItem {
    key: i64,    // 键
    val: Value, // 值
}

#[derive(Debug, Clone)]
pub struct BPNode {
    max_key: i64,
    nodes: Vec<Box<BPNode>>,
    items: Vec<Box<BPItem>>,
    next: Option<Box<BPNode>>,
}

#[derive(Debug, Clone)]
pub struct BPTree {
    ktype: i64,
    root: Box<BPNode>,
    width: i64,
    // 节点的宽度（即每个节点最多有多少个子节点）
    halfw: i64,        // 节点的一半宽度
}

impl BPNode {
    pub fn new_leaf_node(width: i64) -> Self {
        let items = Vec::with_capacity((width + 1) as usize);
        BPNode {
            max_key: 0,
            nodes: vec![],
            items,
            next: None,
        }
    }

    pub fn new_index_node(width: i64) -> Self {
        let items = Vec::with_capacity((width + 1) as usize);
        BPNode {
            max_key: 0,
            nodes: vec![],
            items,
            next: None,
        }
    }
    pub fn set_value(&mut self, key: i64, val: Value) {
        let item = BPItem {
            key,
            val,
        };
        let num = self.items.len();
        if num < 1 {
            self.max_key = item.key;
            self.items.push(Box::new(item));
        } else if key < self.items[0].key {
            self.items.insert(0, Box::new(item));
        } else if key > self.items[num - 1].key {
            self.max_key = item.key;
            self.items.push(Box::new(item));
        } else {
            for i in 0..num {
                if self.items[i].key > key {
                    self.items.insert(i, Box::new(item));
                    return;
                } else if self.items[i].key == key {
                    self.items[i] = Box::new(item);
                    return;
                }
            }
        }
    }

    pub fn add_child(&mut self,  child:Box<BPNode>) {
        let num = self.nodes.len();
        if num < 1 {
            self.max_key = child.max_key;
            self.nodes.push(child);
            return;
        } else if child.max_key < self.nodes[0].max_key {
            self.nodes.insert(0, child);
            return;
        } else if child.max_key > self.nodes[num - 1].max_key {
            self.max_key = child.max_key;
            self.nodes.push(child);
            return;
        }

        for i in 0..num {
            if self.nodes[i].max_key > child.max_key {
                self.nodes.insert(i, child);
                return;
            }
        }
    }
}

impl BPTree {
    pub fn new(width: i64) -> Self {
        let width = if width < 3 { 3 } else { width };
        let root = Box::new(BPNode::new_leaf_node(width));
        let halfw = (width + 1) / 2;
        BPTree { ktype: 0, root, width, halfw }
    }
    pub fn get_data(&self) -> HashMap<i64, Box<dyn Any>> {
        // 由于 Rust 的所有权和生命周期的规则，我们不需要显式地进行锁定和解锁操作
        // Rust 会在适当的时候自动进行这些操作。

        // 调用 getData 方法获取根节点中的所有数据
        return self.get_data_from_node(&self.root);
    }

    pub fn get_data_from_node(&self, node: &BPNode) -> HashMap<i64, Box<dyn Any>> {
        let mut data: HashMap<i64, Box<dyn Any>> = HashMap::new();

        if !node.nodes.is_empty() {
            for child_node in &node.nodes {
                data.insert(child_node.max_key, Box::new(self.get_data_from_node(child_node)));
            }
        } else {
            for item in &node.items {
                data.insert(item.key, Box::new(item.val.clone()));
            }
        }
        return data;
    }

    pub fn set(&mut self, key:i64, value: Value){
        self.set_value(None, self.root.clone(), key, value);
        return;
    }

    pub fn set_value(&mut self, parent: Option<&mut BPNode>, mut node: Box<BPNode>, key: i64, value: Value) {
        if node.nodes.is_empty() {
            // 直接处理叶子节点的情况
            node.set_value(key, value.clone());
            let node_new = self.split_node(&mut node);
            match node_new {
                Some(node_new) => {
                    if let Some(parent) = parent {
                        parent.add_child(node);
                        parent.add_child(Box::new(node_new));
                    } else {
                        let mut new_root = BPNode::new_index_node(self.width);
                        new_root.add_child(node);
                        new_root.add_child(Box::new(node_new));
                        self.root = Box::new(new_root);
                    }
                },
                None => {
                    if parent.is_none() {
                        self.root = node;
                    }
                },
            }
        } else {
            // 先克隆子节点的引用，避免同时持有可变和不可变引用
            let mut child_index = 0;
            let child_node_clone = node.nodes[child_index].clone();

            while child_index < node.nodes.len() && key > node.nodes[child_index].max_key {
                child_index += 1;
            }
            if child_index == node.nodes.len() {
                child_index -= 1;
            }
            // 现在可以安全地进行递归调用，因为不再同时持有可变和不可变引用
            self.set_value(Some(&mut *node), child_node_clone, key, value);
        }
    }

    // pub fn set_value(&mut self, parent:Option<&mut BPNode>, node: Box<BPNode>, key:i64, value: Value){
    //     let i=0;
    //     for i in 0..node.nodes.len(){
    //         if key<=node.nodes[i].max_key||i==node.nodes.len()-1{
    //             self.set_value(Some(node),node.nodes[i],key,value);
    //             break
    //         }
    //     }
    //     if node.nodes.is_empty() {
    //         node.set_value(key,value.clone())
    //     }
    //     let node_new=self.split_node(node);
    //     if let Some(node_new)=node_new{
    //         if parent.is_none(){
    //             let mut parent=BPNode::new_index_node(self.width);
    //             parent.add_child(Box::new(node));
    //             self.root=Box::new(parent);
    //         }
    //
    //         if let Some(parent)=parent{
    //             parent.add_child(Box::new(node_new))
    //         }
    //     }
    // }
    pub fn split_node(&self, mut node: &mut BPNode) -> Option<BPNode> {
        let num = node.nodes.len();
        if num > self.width as usize {
            let halfw = self.width / 2 + 1;
            let mut node2 = BPNode::new_index_node(self.width);
            node2.nodes = node.nodes.split_off(halfw as usize);
            node2.max_key=node2.nodes.last().unwrap().max_key;

            node.max_key=node.nodes.last().unwrap().max_key;
            return Some(node2);
        }else if node.items.len()> self.width as usize {
            let halfw = self.width / 2 + 1;
            let mut node2 = BPNode::new_leaf_node(self.width);
            node2.items = node.items.split_off(halfw as usize);
            node2.max_key = node2.items.last().unwrap().key;

            // Modify original node data
            node.next = Some(Box::new(node2.clone()));
            node.max_key = node.items.last().unwrap().key;

            return Some(node2);
        }
        return None
    }
}