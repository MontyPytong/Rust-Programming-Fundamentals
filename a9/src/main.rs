// Topic : Data managemenet usint tuples
//
// Requirements : 
// * Print whatever the y-value of a cartesian coordinate is
// greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuples
fn cartesian_coordonate() -> (i32, i32,){
    (1, 2)
}

fn main() {
    // * Destrucuture the return value into two varriables
    let (coordonate_x,coordonate_y) = cartesian_coordonate();
    // * Use an if..else if..else vlocl to determine what to print
    if coordonate_y > 5 {
        println!(">5");
    }else if coordonate_y < 5{
        println!("<5");
    }else {
        println!("=5");
    }
}
