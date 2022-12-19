// Topic: Ownership

// Requirements:
//     +Print out the quantity and id number of a grocery item

// Notes:
//     + Use a struct for the grocery item
//     + Use two i32 for the quantity and id number
struct Grocery{
    quantity:i32,
    id:i32, 
}
//     + Create a function to display quantity 
fn display_quantity(grocery_quantity:&Grocery){
    println!("Quantity = {:?}",grocery_quantity.quantity );
}
//     + Create a function to display the id number 
fn display_id(grocery_id:&Grocery){
    println!("Id = {:?}", grocery_id.id);
}

fn main(){
    let list = Grocery{
        quantity:2,
        id:225,
    };
    display_quantity(&list);
    display_id(&list);
}