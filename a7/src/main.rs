// Topic : Working with an enum
//
// Program requirements :
// 
// Notes:
// * Use an enum with colot names as variants
enum Colors {
    Red,
    Blue,
}
// * Use a function tp otint the color name
// * The function must use the enum as a parameter
fn print_color(my_color:Colors){
    // * Use a match expression to determinate which color name to print
    match my_color{
        Colors::Red=>println!("RED"),
        Colors::Blue=>println!("BLUE"),
    }
}

fn main() {
    print_color(Colors::Red);

}