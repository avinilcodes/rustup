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

    // Bitwise operator
    let mut value = 0b0111_0101u8;
    println!("value of value is {}", value);
    println!("value of value is {:08b}", value);

    // NOT
    value = !value;
    println!("value of value is {:08b}", value);

    // AND
    value = value & 0b1111_0111u8;
    println!("value of value is {:08b}", value);
    println!("value of value is {}", value & 0b0100_0000);

    // OR
    value = value | 0b0100_0000u8;
    println!("value of value is {:08b}", value);

    // XOR
    value = value ^ 0b0101_0101u8;
    println!("value of value is {:08b}", value);

    // SHIFT
    value = value << 4;
    println!("value of value is {:08b}", value);

    value = value >> 4;
    println!("value of value is {:08b}", value);

    // Boolean
    let h = true;
    let t = false;
    println!("value of h is {}, value of t is {}", h, t);
    println!("not of h {}", !h);
    println!("h AND t is {}", h&t);
    println!("h OR t is {}", h|t);
    println!("h XOR t is {}", h^t);

    let n = (h^t) | (h&t);
    println!("value of n is {}", n);

    // short-cutting OR and AND
    let val = (h^t) || panic!();
    // val = (h^t) && panic!(); // Panics here
    println!("val {}", val);
    
    // Comparision operators
    println!("value of h is {}, value of t is {}", h, t);
    println!("h EQUALS t {}", h == t);
    println!("h NOT EQUALS t is {}", h != t);
    println!("h GREATER THAN t is {}", h>t);
    println!("h LESS THAN t is {}", h<t);
    println!("h GREATER THAN EQUAL TO t is {}", h>=t);
    println!("h LESS THAN EQUAL TO t is {}", h<=t);

    // char literal

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}\n", letter, number, finger);


    // Challenge
    let q = 13;
    let w = 2.3;
    let e: f32 = 120.0;
    
        /* YOUR CODE GOES HERE */
    let average = (q as f64 + w + e as f64)/3.0;
    
    assert_eq!(average, 45.1);
    println!("Test passed!");

}
