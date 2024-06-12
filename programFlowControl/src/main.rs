fn main() {
    let x = 3;
    if x +1 == 4{
        println!("x+1 is 4");
    }
    let y = 3;

    if x > y {
        println!("x is greater than y");
    }else if x<y{
        println!("x is less than y");
    }else{
        println!("x is equal to y")
    }

    let make_z_odd = true;
    let z = if make_z_odd {1} else {2}; // else {2.0} //error `if` and `else` have incompatible types

    // if make_z_odd{
    //     z=1;
    // }else{
    //     // z=2;// error used binding `z` is possibly-uninitialized
    //     z = 2;
    // }

    println!("z is {}",z);

}
