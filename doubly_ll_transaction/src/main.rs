#[derive(Debug,Clone)]
enum TransactionType{
    Deposition,
    Withdrawal
}
#[derive(Debug,Clone)]
struct Transaction{
    id: u32,
    name:TransactionType,
    amount: u32,
}

use crate::TransactionType::{Deposition,Withdrawal};

#[derive(Debug,Clone)]
struct Node{
    transaction: Transaction,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}
impl Node{
    pub fn new(transaction: Transaction)-> Node{
        Node{
            transaction,
            prev:None,
            next: None,
        }
    }
    pub fn add_transaction(&mut self, transaction:Transaction){
        let mut current= self;
        loop{
            if let Some(ref mut node_present)= current.next{
                current=node_present;
            }else{
                let temp= Box::new(Node{
                        transaction,
                        prev: Some(Box::new(Node{
                            transaction: current.transaction.clone(),
                            next:None,
                            prev: current.prev.take(),
                        })),
                        next: None,
                    });
                    current.next= Some(temp);
                    break;
            }
            
        }
    }

    pub fn show_all(&mut self){
        println!("Printing all transactions: ");
        let mut current= self;
        loop{
            if let Some(ref mut node_present)=current.next{
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

    head_trans.show_all();
}