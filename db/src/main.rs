use crate::server::bptree::{BPTree, Value};

mod server;



fn main() {

   let mut tr =  BPTree::new(3);


    tr.set(10, Value::Int(1));
    tr.set(1, Value::Int(1));
    tr.set(2, Value::Int(1));
    tr.set(3, Value::Int(1));
    tr.set(4, Value::Int(1));
    tr.set(5, Value::Int(1));
    tr.set(6, Value::Int(1));
    tr.set(7, Value::Int(1));

    println!("{:#?}",tr.get_data())





}
