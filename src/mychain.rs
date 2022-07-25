use::chrono::prelude::*;
// use serde_json::to_string;
use sha2::{Digest,Sha256};
use std::fmt::Write;
use serde_derive::{Deserialize, Serialize};

use std::vec;

#[derive(Debug)]
struct Block{
    header: BlockHeader,
    transaction: Vec<Transaction>,
    count:u32
}

#[derive(Debug,Serialize, Deserialize)]
struct BlockHeader{
    timestamp: i64,
    nonce: u64,
    prev_hash: String,
    merkle_hash: String,
    difficulty: u32,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
struct Transaction{
    sender: String,
    receiver: String,
    amount:f32
}

#[derive(Debug)]
 pub struct Chain<'a>{
    chain: Vec<Block>,
    miner_addr:&'a str,
    reward:f32,
    difficulty:u32,
    curr_trans:Vec<Transaction>
}
// generating genesis block by impl chain

   impl<'a> Chain<'a> {
       
pub fn new(miner_addr:&'a str,difficulty:u32 ,reward:f32)-> Self {
        
        let mut chain = Chain{
            chain: vec![],
            miner_addr,
            reward,
            difficulty,
            curr_trans:vec![]
        };

        chain.generate_new_block();
        chain
    }


   pub fn add_transaction(&mut self, sender:String, rec:String, amount:f32) ->Result<(),String> {
        
        let trans = Transaction{
            sender:sender,
            receiver:rec,
            amount
        };

        self.curr_trans.push(trans);

        Ok(())
    }

      pub fn generate_new_block(&mut self) -> Result<(),String>{

        // Instance of header
        let header = BlockHeader{
            timestamp:Utc::now().timestamp(),
            nonce : 0,
            prev_hash : self.get_prev_hash(),
            merkle_hash : String::new(),
            difficulty : self.difficulty,
        };


        // Instance of transactions
         let reward_trans = Transaction{
            sender:String::from("Root"),
            receiver:self.miner_addr.to_owned(),
            amount:100.0
         };

        //  Instance for vector to store instance of transactions



        // Instance of block
        
        let mut block = Block{
            header,
            transaction:vec![],
            count:0
        };
        block.transaction.push(reward_trans);
        block.transaction.append(&mut &mut self.curr_trans);
        block.count = block.transaction.len() as u32;
        block.header.merkle_hash= Self::get_merkle(block.transaction.clone());

        Self::proof_of_work(&mut block.header);


        println!("{:#?}",block);

        self.chain.push(block);

        
        Ok(())
      }


      fn proof_of_work(header:&mut BlockHeader) {
          
        // we find the nonce in this process
        

        // using the slice method to get the first few characters of the hash
        // diffiuscy is the number of characters we want to get

        loop {
            let hash = Self::Hash(header);
            let slice = &hash[..header.difficulty as usize];

            let parsed_slice = slice.parse::<u32>();

            match parsed_slice {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    }else{
                        println!("{:?}",hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
        




      }


      pub fn change_difficulty(&mut self,new_diff:u32) -> Result<(),String>{
        self.difficulty = new_diff;

        Ok(())
      } 

      
      pub fn change_reward(&mut self,new_reward:f32) -> Result<(),String>{
        self.reward = new_reward;

        Ok(())
      } 

      fn get_merkle(curr_trans:Vec<Transaction>) ->String {
          let mut merkle = Vec::new();

          for t in &curr_trans {
            let hash = Self::Hash(t);
            merkle.push(hash)
          }

          if merkle.len()%2==1{
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
          }

        //   loop
        while merkle.len()>1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);

            h1.push_str(&mut h2);

            let combined_hash = Self::Hash(&h1);
            merkle.push(combined_hash)



        }
          merkle.pop().unwrap()
      }

      fn  get_prev_hash(&self) -> String{

        // prev_block 
        
        let prev_block = match self.chain.last(){
              // is there is a block 
            Some(block )=> {
                block
            },
            // if no block => genesis block 
            None =>  return String::from("0000000000000000000000000000000000000000000000000000000000000000"),

        };
      
        // generate the hash 
       Self::Hash(&prev_block.header)
   }

   fn Hash<T:serde::ser::Serialize>(item:&T) ->String {
       

    // Blockheader and Transaction to be hashed
    // they can be converted to string using serde crate
    
    let item_string = serde_json::to_string(&item).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(item_string.as_bytes());
    let result = hasher.finalize();
//    converting the hash to vector
    let res_vec = result.to_vec();

    // converting the vector to string using write!


    Self::hex_to_string(res_vec)
    }
    //    a function to convert vector to string
    fn hex_to_string (item:Vec<u8>) -> String{
        let mut s = String::new();
        for b in item {
            write!(&mut s,"{:?}",b).expect("Unable to convert");
        }
        s
    }
   }




