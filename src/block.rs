use crate::errors::Result;
use crate::blockchain::Blockchain;
use crate::config;
use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;


// Define the structure of the individual block
#[derive(Debug,Clone)]
pub struct Block{
 timestamp:u128,
 transactions: String,
 prev_block_hash: String,
 hash: String,
 height: usize,
 nonce: i32,
}

// Implement the functions of the block
impl Block {
 pub fn get_hash(&self) -> String{
  self.hash.clone()
 }
 // New Genesis Block
 pub fn new_genesis_block()->Block{
  Block::new_block(String::from("genesis block"),String::new(),0).unwrap()
 }


 pub fn new_block(data: String, prev_block_hash: String, height:usize) -> Result<Block> {
  let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis(); // Captures the current timestamp in milliseconds since the UNIX epoch
  let mut block = Block {
   timestamp: timestamp,
   transactions: data,
   prev_block_hash,
   hash: String::new(),
   height,
   nonce:0,
  };
  block.run_proof_of_work()?;
  Ok(block)
 }

 fn run_proof_of_work(&mut self) -> Result<()>{
  info!("Mining the block...");
  while !self.validate()?{
   self.nonce += 1;
  }
  let data = self.prepare_hash_data()?;
  let mut hasher = Sha256::new();
  hasher.input(&data[..]);
  self.hash = hasher.result_str();
  Ok(())
 }

 fn prepare_hash_data(&self) -> Result<Vec<u8>>{
  let content = (
   self.prev_block_hash.clone(),
   self.transactions.clone(),
   self.timestamp,
   config::TARGET_HEXT,
   self.nonce
  );
  let bytes = bincode::serialize(&content)?;
  Ok(bytes)
 }

 fn validate(&self) -> Result<bool>{
  let data = self.prepare_hash_data()?;
  let mut hasher = Sha256::new();
  hasher.input(&data[..]);
  let mut vec1: Vec<u8> = vec![];
  vec1.resize(config::TARGET_HEXT,'0' as u8);
  Ok(&hasher.result_str()[0..config::TARGET_HEXT] == String::from_utf8(vec1)?)
 }
}

#[cfg(test)]
mod tests{
 use super::*;

 #[test]
 fn test_blockchain(){
  let mut b = Blockchain::new();
  b.add_block("data".to_string());
  b.add_block("data2".to_string());
  b.add_block("data3".to_string());
  dbg!(b);
 }
}