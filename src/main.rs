extern crate ship_wreck;

fn main() {
    let t = ship_wreck::string_return();

    println!("Printing string...");
    println!("{}", t);

    let c = ship_wreck::get_default_client();
    println!("{:?}", c);
}
