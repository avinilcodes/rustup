fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket fuel {} and length {}", rocket_fuel,length);
}


fn process_fuel(propellant: &mut String) ->  usize{
    propellant.push_str("is highly flammable");
    println!("propellant is {}", propellant); // propellant here gets the reference
    // and rocket_fuel has the ownership
    let length = propellant.len();
    length
}