#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused)]

use crate::linked_list::List;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Account(u64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct TxId(u64);

impl TxId {
    fn next(&self) -> TxId {
        TxId(self.0 + 1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Transaction {
    txid: TxId,
    from: Account,
    to: Account,
    amount: u64,
}

struct Ledger {
    latest_txid: TxId,
    transactions: List<Transaction>,
}

// NOTE: Use iterator methods as much as possible
// Refer https://doc.rust-lang.org/std/iter/trait.Iterator.html

impl Ledger {
    fn new() -> Ledger {
        todo!()
    }

    /// Add the transaction to the ledge with Initiated state
    fn add_transaction(self, from: Account, to: Account, amount: u64) -> Ledger {
        // This is a functional style implementation but this function takes the ownership
        // of the Ledger(self) and produces a new ledger for simplicity
        todo!()
    }


    fn max_amount_transaction(&self) -> Option<&Transaction> {
        // Why Option<&Transaction>?
        todo!()
    }

    // Total amount of the entire ledger
    fn total_amount_being_transacted(&self) -> u64 {
        todo!()
    }

    fn total_amount_transacted_by(&self, account: Account) -> u64 {
        todo!()
    }

    fn total_amount_received_by(&self, account: Account) -> u64 {
        todo!()
    }

    fn number_of_transactions_made_by(&self, account: Account) -> u32 {
        todo!()
    }

    fn number_of_transactions_received_by(&self, account: Account) -> u32 {
        todo!()
    }
    
    // BONUS

    // You can use references from the internet or code generators to deal with file IO
    // This topic has too much details but all you have to know is that doing IO is not
    // a guaranteed success so the operations results in a Result which can de either Ok or an Err
    
    /// Use csv format to store the data in a given file path
    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        todo!()
    }

    // Parse csv data based on how you saved the ledger to the file
    fn load_from_file(path: &str) -> std::io::Result<Ledger> {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use crate::ledger;

    use super::*;

    fn test_ledger_basic() -> Ledger {
        let ledger = Ledger::new();
        let ledger = ledger.add_transaction(Account(1), Account(2), 100);
        let ledger = ledger.add_transaction(Account(2), Account(3), 200);
        ledger.add_transaction(Account(3), Account(4), 300)
    }

    fn test_ledger_complex() -> Ledger {
        let mut ledger = test_ledger_basic();

        for i in 0..2 {
            ledger = ledger.add_transaction(Account(1), Account(2), 100);
            ledger = ledger.add_transaction(Account(1), Account(3), 200);
        }
        ledger
    }

    #[test]
    fn test_ledger_new() {
        let ledger = Ledger::new();
        assert_eq!(ledger.latest_txid, TxId(0));
        assert_eq!(ledger.transactions.len(), 0);
    }

    #[test]
    fn test_ledger_add_transaction() {
        let ledger = Ledger::new();
        let ledger = ledger.add_transaction(Account(1), Account(2), 100);
        assert_eq!(ledger.latest_txid, TxId(1));
        assert_eq!(ledger.transactions.len(), 1);
    }

    #[test]
    fn test_ledger_max_amount_transaction() {
        let ledger = test_ledger_basic();
        let max_amount_transaction = ledger.max_amount_transaction();
        assert_eq!(max_amount_transaction.unwrap().amount, 300);
    }

    #[test]
    fn test_total_amount_being_transacted() {
        let ledger = test_ledger_basic();
        let total_amount = ledger.total_amount_being_transacted();
        assert_eq!(total_amount, 600);
    }

    #[test]
    fn test_total_amount_being_transacted_by() {
        let ledger = test_ledger_complex();
        assert_eq!(ledger.total_amount_transacted_by(Account(1)), 700);
    }

    // You got the idea write other tests too
}
        