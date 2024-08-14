use std::ops::Add;


fn add<T>(a:T,b:T)->T
where T: Add<Output = T>{
    a+b
}

#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}

impl<T> Add for Point<T>
where T:Add<Output = T>
{
    type Output= Point<T>;
    fn add(self, other: Point<T>)-> Point<T>{
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
    println!("Adding struct Point Integers: {:?}", add(p1,p2));

    let p1= Point{
        x:5.5, 
        y:0.0,
    };
    let p2= Point{
        x:0.0,
        y:5.5,
    };
    println!("Adding struct Point Float: {:?}", add(p1,p2));

}