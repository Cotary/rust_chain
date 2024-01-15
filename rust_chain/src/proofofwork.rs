use crate::block::Block;
use crate::utils::int_to_hex;
use std::{fmt, ops::Shl};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
const MAX_NONCE: i64 = std::i64::MAX;
const TARGET_BITS: u8 = 24;
pub struct ProofOfWork{
    block:&'static Block,
    target:u128,

}

impl ProofOfWork {
   pub fn new(block: &'static Block) -> Self {
        let target = 1u128 << (255 - TARGET_BITS);
        Self {block, target }
    }
    pub fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        vec![
            &self.block.prev_block_hash,
            &self.block.data,
            self.block.timestamp.to_string().as_bytes(),
            &TARGET_BITS.to_be_bytes(),
            &nonce.to_be_bytes(),
        ]
            .concat()
    }
    pub fn run(&mut self) -> (i64, Vec<u8>) {
        let mut hash: u128;
        let mut nonce = 0;
        println!("Mining the block containing {:?}", self.block.data);
        loop {
            let data = self.prepare_data(nonce);
            hash = Self::hash(&data);
            print!("\r{:x}", hash);
            if hash < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n\n");
        (nonce, hash.to_be_bytes().to_vec())
    }

    pub  fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        Self::hash(&data)< self.target
    }
       fn hash<T: Hash>(t: &T) -> u128 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish() as u128
    }
}

