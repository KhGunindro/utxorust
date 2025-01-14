use crate::block::Block;
use crate::errors::Result;
use crate::config;
// Define the structure of the blockchain 
#[derive(Debug)]
pub struct Blockchain{
 blocks: Vec<Block>
}

// Implementing the blockchain
impl Blockchain{
 pub fn new()-> Blockchain{
  Blockchain{
   blocks: vec![Block::new_genesis_block()]
  }
 }
 pub fn add_block(&mut self, data:String)->Result<()>{
  let prev = self.blocks.last().unwrap();
  let new_block = Block::new_block(data, prev.get_hash(), config::TARGET_HEXT)?;
  self.blocks.push(new_block);
  Ok(())
 }
}