use crate::block::{Block, new_genesis_block};
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data.as_bytes().to_vec(), prev_block.hash.clone());
        self.blocks.push(new_block);
    }
}

fn new_block_chain()->Blockchain{
    let genesis_block = new_genesis_block();
    Blockchain { blocks: vec![genesis_block] }
}