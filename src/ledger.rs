use crate::linked_list::List;

enum State {
    Initiated,
    Confirmed,
}

struct Transaction {
    from: String,
    to: String,
    amount: f64,
    state: State,
}

struct Ledger {
    transactions: List<Transaction>,
}

impl Ledger {
    fn new() -> Ledger {
        todo!()
    }

    fn add_transaction(self, from: String, to: String, amount: f64) -> Ledger {
        // This is a functional style implementation but this function takes the ownership
        // of the Ledger(self) and produces a new ledger for simplicity
        todo!()
    }


    // You can use references from the internet or code generator to deal with file IO
    // This topic has too much details but all you have to know is that doing IO is not
    // a guaranteed success so the operations results in a Result which can de Ok or an Err

    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        todo!()
    }

    fn load_from_file(path: &str) -> std::io::Result<Ledger> {
        todo!()
    }

    fn max_amount_transaction(&self) -> Option<&Transaction> {
        // Why Option<&Transaction>?
        todo!()
    }

    fn confirmed_transactions(&self) -> List<&Transaction> {
        todo!()
    }

    fn initiated_transactions(&self) -> List<&Transaction> {
        todo!()
    }

    fn total_amount_being_transacted(&self) -> f64 {
        todo!()
    }

    fn total_amount_transacted_by(&self, name: &str) -> f64 {
        todo!()
    }

    fn number_of_transactions_made_by(&self, name: &str) -> u32 {
        todo!()
    }
}

// Write appropriate unit tests