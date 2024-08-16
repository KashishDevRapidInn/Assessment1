#[derive(Debug)]
enum TransactionType{
    Deposition,
    Withdrawal
}
#[derive(Debug)]
struct Transaction{
    id: u32,
    name:TransactionType,
    amount: u32,
}

use crate::TransactionType::{Deposition,Withdrawal};

#[derive(Debug)]
struct Node{
    transaction: Transaction,
    next: Option<Box<Node>>
}
impl Node{
    pub fn new(transaction: Transaction)-> Node{
        Node{
            transaction,
            next: None,
        }
    }
    pub fn add_transaction(&mut self, transaction:Transaction){
        let mut current= self;
        loop{
            if let Some(ref mut node_present)= current.next{
                current=node_present;
            }else{
                current.next= Some(Box::new(Node::new(transaction)));
                break;
            }
        }
    }

    pub fn remove_transaction(&mut self)-> Transaction{
   
        let mut current= self;
        
        loop{
           if let Some(mut node_present) = current.next.take() {
                if node_present.next.is_none() {
                    let value = node_present.transaction;
                    current.next = None;
                    return value;
                } else {
                    current.next = Some(node_present);
                    current = current.next.as_mut().unwrap();
                }
            }
        }
    
    }
    
    pub fn show_all(&self){
        println!("Printing all transactions: ");
        let mut current= self;
        loop{
            if let Some(ref node_present)=current.next{
                println!("Transaction Id : {:?}, Transaction Type: {:?}, Transaction Amount: {:?} ", current.transaction.id, current.transaction.name, current.transaction.amount);
                current=node_present;
            }else{
                break;
            }
        }
         println!("Transaction Id : {:?}, Transaction Type: {:?}, Transaction Amount: {:?} ", current.transaction.id, current.transaction.name, current.transaction.amount);
    }
}

fn main(){
    let trans1= Transaction{
        id: 1,
        name: Deposition,
        amount: 500
    };
    let mut head_trans= Node::new(trans1);
    println!("{:?}", head_trans);
    let trans2= Transaction{
        id: 2,
        name: Withdrawal,
        amount:200,
    };
    head_trans.add_transaction(trans2);
    let trans3= Transaction{
        id: 3,
        name: Deposition,
        amount:2000,
    };
    head_trans.add_transaction(trans3);
    println!("{:?}", head_trans);

    let popped_transaction= head_trans.remove_transaction();
    println!("{:?}", popped_transaction);

    head_trans.show_all();
}