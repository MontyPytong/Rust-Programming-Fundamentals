// Topic : Implementing functionality with the impl keyword

// Requirements:
//     + Print the characteristics of a shipping box
//     + Must include dimensions, weight and color
    
//     Notes:
//         + Use a struct to encapsulate the box characteristics

//         + Use an enum for the box color
enum Color{
    Red,
    Blue,
    Yellow
}
impl Color{
    fn print(&self){
        match self{
            Color::Red=>println!("RED"),
            Color::Blue=>println!("BLUE"),
            Color::Yellow=>println!("YELLOW"),
        }
    }
}

struct Dimensions{
    width:f64,
    height:f64,
    depth:f64,
}

impl Dimensions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}


struct Box{
    dimensions:Dimensions,
    weight:f64,
    color:Color,
}

//         + Implement functionality on the box struct to create a new box
impl Box{
   fn new(weight:f64, color:Color, dimensions:Dimensions)->Self{
    Self{
        weight,
        color,
        dimensions,
    }
   }
   fn print(&self){
    self.color.print();
    self.dimensions.print();
    println!("weight:{:?}", self.weight);
    
   }
}
//         + Implement functionality on the box struct to print the characteristics

fn main(){
    let small_box = Dimensions{
        width:1.0,
        height:2.0,
        depth:3.0
    };
    let new_box = Box::new(5.0, Color::Red, small_box);
    new_box.print();
}