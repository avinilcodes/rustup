fn main() {
    // let planet = "Earth"; 
    // if true{
    //     // let planet = "Earth"; // cannot find value `planet` in this scope
    //     println!("planet is {}", planet);
    // }
    // println!("planet is {}", planet); // cannot find value `planet` in this scope

    // Shadowing
    let planet = "Earth"; 
    println!("planet is {}", planet);
    // let planet = "Mars";
    {
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);

    // String data type

    let mut message = String::from("Earth");
    println!("Message is {}", message);
    message.push_str(" is home.");
    println!("Message is {}", message);

    // Ownership transfer

    // let rocket_fuel = String::from("RP-1");
    // // process_fuel(rocket_fuel); // borrow of moved value: `rocket_fuel`
    // process_fuel(rocket_fuel.clone());
    // println!("{}", rocket_fuel); // borrow of moved value: `rocket_fuel`

    let mut rocket_fuel = String::from("RP-1");
    rocket_fuel = process_fuel(rocket_fuel);
    println!("{}", rocket_fuel);
}

fn process_fuel(mut propellant: String) -> String{
    propellant = String::from("PNG");
    println!("{}", propellant); // propellant PNG rocket_fuel PNG return statement
    propellant
}
