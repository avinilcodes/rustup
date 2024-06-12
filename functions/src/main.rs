
fn main() {
    println!("Hello, world!");
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    // say_number(x); // mismatched types  expected `i32`, found `u8`
    say_number(x as i32);

    // function return values
    println!("square of is {:?}", square(13));

    // challenge
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
    // println!("0 celsius to fahrenheit is {}", celsius_to_fahrenheit(4.0));

}


fn say_hello(){
    println!("Hello!");
    say_number(5);
}

fn say_number(number : i32){
    println!("number is {}", number);
}

fn say_the_sum(a:u8,b:u8){
    println!("a + b equals {}", a+b);
}

fn square(x:i32) ->(i32,i32){
    println!("executing square");
   return (x,x*x);
   println!("end of square");
}

fn celsius_to_fahrenheit(c :f64) -> (f64){
    (c*1.8)+32.0
}