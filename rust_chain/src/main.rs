use crate::blockchain::new_block_chain;

mod blockchain;
mod block;
mod proofofwork;
mod utils;

fn main() {
    let mut bc=new_block_chain();
    bc.add_block(String::from("Send 1 BTC to Ivan"));
    bc.add_block(String::from("Send 2 more BTC to Ivan"));

    for (k,v)in bc.blocks.iter().enumerate(){
        println!("{:#?},{:#?}",k,v);
    }

}
