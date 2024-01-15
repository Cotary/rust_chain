
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug)]
pub struct  Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Block {
    pub  fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block {
            timestamp,
            data,
            prev_block_hash,
            hash: Vec::new(),
        };
        block.set_hash();
        block
    }
    pub  fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.input(&self.prev_block_hash);
        hasher.input(&self.data);
        hasher.input(self.timestamp.to_string().as_bytes());
        //hasher.result(&mut self.hash); todo 这段代码为啥不行
        self.hash=hasher.result_str().as_bytes().to_vec();

    }
}

pub fn new_genesis_block() -> Block {
    return Block::new(b"Genesis Block".to_vec(), vec![]);
}