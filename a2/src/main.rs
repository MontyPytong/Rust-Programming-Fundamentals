// Notes:
// Use a function to add to numbers toghether
fn add(a:i32, b:i32)->i32{
    a+b
}
// Use a function to display the result
// Use the "{:?}" token in the println macro to display the result
fn display_result(result:i32){
    println!("{:?}", result);
}


fn main() {
    let result = add(2,2);
    display_result(result);
    
}
