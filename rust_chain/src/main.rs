use crate::blockchain::{Blockchain, new_block_chain};

mod blockchain;
mod block;
mod proofofwork;
mod utils;

fn main() {
    let mut bc=Blockchain::new();
    bc.add_block(String::from("Send 1 BTC to Ivan"));
    bc.add_block(String::from("Send 2 more BTC to Ivan"));

    for (k,v)in bc.blocks.iter().enumerate(){
        println!("{:#?},{:#?}",k,v);
    }

}
