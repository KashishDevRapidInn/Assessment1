#[derive(Debug)]
#[derive(Clone)]
 #[derive(PartialEq)]
struct Node{
    data: u32,
    next: Option<Box<Node>>,
}
impl Node{
    pub fn new(data:u32)->Node{
        Node{
            data,
            next: None
        }
    }
    pub fn push_element(&mut self, value: u32){
        let mut current = self;
        loop{
            if let Some(ref mut next)= current.next{
                current=next;
            }else{
                current.next = Some(Box::new(Node::new(value)));
                    break;
            }
        }
    }
    pub fn push_element_start(&mut self, value: u32){
        let mut temp=Node::new(value);
        temp.next= Some(Box::new(
            Node {
            data: self.data,
            next: self.next.take(),
        }
        ));
        *self= temp
    }
    pub fn pop_element(&mut self)->u32{
        let mut current= self;
        
        loop{
           if let Some(mut node_present) = current.next.take() {
                if node_present.next.is_none() {
                    let value = node_present.data;
                    current.next = None;
                    return value;
                } else {
                    current.next = Some(node_present);
                    current = current.next.as_mut().unwrap();
                }
            }
        }
    }
    pub fn pop_element_start(&mut self)->u32{
        
        let value= self.data;
        if let Some(node_present)= self.next.take(){
            *self= *node_present;
        }
        return value;
    }
    pub fn show(&self){
        let mut current= self;
        while let Some(ref next)= current.next{
            print!("{}->", current.data);
            current=next;
        }
        println!("{}->None", current.data);
    }

}

fn main() {
    let mut head= Node::new(1);
    head.push_element(2);
    head.push_element(3);
    head.push_element_start(0);
    println!("{:?}", head);
    let num= head.pop_element();
    println!("{}", num);
    let num= head.pop_element_start();
    println!("{}", num);
    println!("{:?}", head);
    head.show();
}
