// Looping using while statement
//
// Program requirements :
// Count down from 5 to 1 , displays the countdown.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use the break to exit the loop

fn main() {

    let mut number = 5;
    while number > 0 {
        println!("{}", number);
        number = number -1;
    }
    println!("done!");
}
