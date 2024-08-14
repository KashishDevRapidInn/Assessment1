use std::ops::Add;


fn add<T>(a:T,b:T)->T
where T: Add<Output = T>{
    a+b
}

#[derive(Debug)]
struct Point{
    x:u32,
    y:u32,
}

impl Add for Point{
    type Output= Point;
    fn add(self, other: Point)-> Point{
        Point { 
            x: self.x+ other.x, 
            y: self.y+ other.y
        }
    }
}
fn main(){
    println!("Adding Integer: {}", add(1,2));
    let f1:f32=1.2;
    let f2:f32= 2.2;
    println!("Adding Float: {}", add(f1,f2));
    
    let p1= Point{
        x:5, 
        y:0,
    };
    let p2= Point{
        x:0, 
        y:5,
    };
    println!("Adding struct Point: {:?}", add(p1,p2));
}