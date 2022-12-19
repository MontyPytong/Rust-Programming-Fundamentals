enum Directions {
    Left,
    Right,
}


fn main() {
    let go = Directions::Left;
    match go {
        Directions::Left => println!("go left"),
        Directions::Right => println!("go right"),
    }
}
