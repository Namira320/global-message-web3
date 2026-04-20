#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Data structure that will store the message details
#[contracttype]
#[derive(Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub author: String,
    pub content: String,
}

// Storage key for the messages data (Max 9 characters for symbol_short)
const MESSAGE_DATA: Symbol = symbol_short!("MESSAGES");

#[contract]
pub struct MessageBoardContract;

#[contractimpl]
impl MessageBoardContract {
    // Function to retrieve all posted messages
    pub fn get_messages(env: Env) -> Vec<Message> {
        // 1. Fetch messages data from instance storage
        return env.storage().instance().get(&MESSAGE_DATA).unwrap_or(Vec::new(&env));
    }

    // Function to post a new message to the board
    pub fn post_message(env: Env, author: String, content: String) -> String {
        // 1. Fetch existing messages from storage
        let mut messages: Vec<Message> = env.storage().instance().get(&MESSAGE_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Create a new message object
        let new_message = Message {
            id: env.prng().gen::<u64>(), // Generate a unique random ID
            author: author,
            content: content,
        };
        
        // 3. Append the new message to the existing list
        messages.push_back(new_message);
        
        // 4. Save the updated list back to storage
        env.storage().instance().set(&MESSAGE_DATA, &messages);
        
        return String::from_str(&env, "Message posted successfully");
    }

    // Function to delete a message based on its ID
    pub fn delete_message(env: Env, id: u64) -> String {
        // 1. Fetch messages data from storage
        let mut messages: Vec<Message> = env.storage().instance().get(&MESSAGE_DATA).unwrap_or(Vec::new(&env));

        // 2. Loop through to find the index of the message to be deleted
        for i in 0..messages.len() {
            if messages.get(i).unwrap().id == id {
                // Remove the message from the vector
                messages.remove(i);

                // Update the storage with the modified list
                env.storage().instance().set(&MESSAGE_DATA, &messages);
                return String::from_str(&env, "Message deleted successfully");
            }
        }

        return String::from_str(&env, "Message not found")
    }
}

mod test;