#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// A structure to store each quote with its ID, text, and author
#[contracttype]
#[derive(Clone)]
pub struct Quote {
    pub quote_id: u64,
    pub text: String,
    pub author: String,
}

// Symbol key for tracking the number of stored quotes
const QUOTE_COUNTER: Symbol = symbol_short!("Q_COUNT");

// Enum used as keys for storing/retrieving quotes from contract storage
#[contracttype]
pub enum Quotes {
    Quote(u64),
}

#[contract]
pub struct DailyQuotesContract;

#[contractimpl]
impl DailyQuotesContract {
    /// Adds a new quote to the storage and returns its unique ID
    pub fn add_quote(env: Env, text: String, author: String) -> u64 {
        // Retrieve current quote count or start at 0
        let mut count: u64 = env.storage().instance().get(&QUOTE_COUNTER).unwrap_or(0);
        count += 1;

        let new_quote = Quote {
            quote_id: count,
            text,
            author,
        };

        // Save the new quote and update the counter
        env.storage().instance().set(&Quotes::Quote(count), &new_quote);
        env.storage().instance().set(&QUOTE_COUNTER, &count);

        count
    }

    /// Retrieves a quote by its unique ID
    pub fn get_quote(env: Env, quote_id: u64) -> Quote {
        env.storage()
            .instance()
            .get(&Quotes::Quote(quote_id))
            .unwrap_or(Quote {
                quote_id: 0,
                text: String::from_str(&env, "No Quote Found"),
                author: String::from_str(&env, "Unknown"),
            })
    }

    /// Returns the total number of quotes stored
    pub fn total_quotes(env: Env) -> u64 {
        env.storage().instance().get(&QUOTE_COUNTER).unwrap_or(0)
    }
}
