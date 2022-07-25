use std::io;


mod mychain;

use mychain::Chain;
fn main() {
    println!("Enter the miner's address");
    let mut miner_addr = String::new();
    io::stdin().read_line(&mut miner_addr);

    println!("Enter the Difficulty");
    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty);

    let parsed_difficulty:u32 = difficulty.trim().parse().unwrap();
    
    println!("Generating genesis block...");

    // generate genesis block

    let mut chain = Chain::new(miner_addr.as_str(),parsed_difficulty,100.0);


    loop {
        
        println!("Menu");
    println!("1. New transaction");
    println!("2. Mine block");
    println!("3. change Difficulty");
    println!("4. Change reward");
    println!("0. Exit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice);




    let parsed_choice:u32 = choice.trim().parse().unwrap();

    match parsed_choice {
        1 => {

            
    let mut sender = String::new();
    let mut rec = String::new();
    let mut amount = String::new();

            // Add a transaction
            println!("Enter sender address");
            
            io::stdin().read_line(&mut sender);

            println!("Enter receiver address");
            
            io::stdin().read_line(&mut rec);

            
            // let a = convert_to_ref(sender);
            println!("Enter amount");
            
            io::stdin().read_line(&mut amount);


            let parsed_amount:f32 = amount.trim().parse().unwrap();

            let res = chain.add_transaction(sender,rec,parsed_amount);

            match res {
                Ok(_) => {println!("Transaction added");
            },
                Err(_) =>{
                    println!("Transaction failed");
                }
            }
                
        },
        2 => {
            // Mine a block
            println!("Generating a block...");
            let res = chain.generate_new_block();

            match res {
                Ok(_) => {println!("Block generated");
            },
                Err(_) =>{
                    println!("Block failed");
                }
            }
                
            },
        3 => {
            // Change difficulty
            let mut new_difficulty = String::new();
            io::stdin().read_line(&mut new_difficulty);

            let parsed_difficulty:u32 = new_difficulty.trim().parse().unwrap();
            let res = chain.change_difficulty(parsed_difficulty);
            match res {
                Ok(_) => {println!("Difficulty changed");
            },
                Err(_) =>{
                    println!("Difficulty change failed");
                }
            }

        },
        4 => {
            // Change reward
            let mut new_reward = String::new();
            io::stdin().read_line(&mut new_reward);

            let parsed_reward:f32 = new_reward.trim().parse().unwrap();
            let res = chain.change_reward(parsed_reward);

            match res {
                Ok(_) => {println!("Reward changed");
            },
                Err(_) =>{
                    println!("Reward change failed");
                }
            }


        },
        _ => {
            // exit

            println!("Exiting...");
            break;
        }
        
    }
    }

    


}


// fn convert_to_Ref<'a>(s:String) ->&'a str {
    
//     s.as_str()
// }