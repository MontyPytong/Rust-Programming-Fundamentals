// Topic : Organizing similar data using struct
//
// Requirements:
// * print the flavor of a dark and its fluid ounces
//
// Notes :
// * Use an enum to create different flavours of drinks
enum Flavour {
    Mar,
    Para,
    Caise,
}
// * Use a struct to store drink flavour and fluid ounce information
struct Drink {
    flavour:Flavour,
    ounces:f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(drink:Drink){
    match drink.flavour{
        Flavour::Mar=>println!("Mar"),
        Flavour::Para=>println!("Para"),
        Flavour::Caise=>println!("Caise"),
    }
    println!("oz : {:?}", drink.ounces)
}
// * Use a match expression to print the drink flavour
fn main(){
    let mar = Drink{
        flavour:Flavour::Mar,
        ounces:6.0
    };
    print_drink(mar);
    let para = Drink{
        flavour:Flavour::Para,
        ounces:7.0
    };
    print_drink(para);
}