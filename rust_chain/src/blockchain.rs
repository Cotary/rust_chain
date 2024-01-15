use crate::block::{Block, new_genesis_block};
#[derive(Debug)]
pub struct Blockchain {
   pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data.as_bytes().to_vec(), prev_block.hash.clone());
        self.blocks.push(new_block);
    }
}

pub fn new_block_chain()->Blockchain{
    let genesis_block = new_genesis_block();
    Blockchain { blocks: vec![genesis_block] }
}