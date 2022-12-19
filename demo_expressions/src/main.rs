// Determines if someone has acces or not to resources

enum Acces {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    //secret file : admins only
    let acces_level = Acces::Guest;
    let can_acces_file = match acces_level{
        Acces::Admin => true,
        _ => false,
    };
    println!("Can acces: {:?}", can_acces_file);
}
