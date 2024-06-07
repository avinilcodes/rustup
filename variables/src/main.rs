fn main() {
    // Integers
    let mut x = -10;
    println!("value x is {}",x);
    x = 20;
    println!("value x is {}",x);

    let y: u8 = 255;
    println!("value of y is {}",y);
    // y += 1;

    // Floating point 
    let z = 10.12345678899123232;
    println!("value z is {}",z);

    let zf: f32= 10.12345678899123232;
    println!("value zf is {}",zf);


    // Arithematic ops
    let j = 10.0;
    let k = 3;
    let l = j / k as f64 + 1.0;
    println!("value of l is {}", l);

    // Formatting print statement also there is fmt package for the same
    let i = 10.0;
    let o = 3.0;
    let p = i/o;
    println!("value of divison is {0:08.3}\n value of i is {1}\n value of p is {0}", p,i);
}
