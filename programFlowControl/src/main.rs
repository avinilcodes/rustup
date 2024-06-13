fn main() {
    // let x = 3;
    // if x +1 == 4{
    //     println!("x+1 is 4");
    // }
    // let y = 3;

    // if x > y {
    //     println!("x is greater than y");
    // }else if x<y{
    //     println!("x is less than y");
    // }else{
    //     println!("x is equal to y")
    // }

    // let make_z_odd = true;
    // let z = if make_z_odd {1} else {2}; // else {2.0} //error `if` and `else` have incompatible types

    // // if make_z_odd{
    // //     z=1;
    // // }else{
    // //     // z=2;// error used binding `z` is possibly-uninitialized
    // //     z = 2;
    // // }

    // println!("z is {}",z);

    // loop 

    let mut count = 1;

    let res = loop {
        if count == 10{
            break count*10;
        }
        count+=1;
        println!("count is {}", count);
    };

    println!("break is working");
    println!("value returned by break is {}", res);

    // while loop

    count = 0;
    let letters = ['a','b','c'];

    while count < letters.len(){
        println!("letter of count is {}", letters[count]);
        count+=1;
    }


    // for loop
    let message = ['h','e','l','l','o'];
    for (index,item) in message.iter().enumerate() {
        println!("item at {} is {}",index, item);
    }

    for number in 0..5{
        println!("number is {}", number)
    }

    let mut matrix = [[1,2,3],
                                [4,5,6],
                                [7,8,9]];

    for row in matrix.iter_mut(){
        for num in row.iter_mut(){
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;
    /* YOUR CODE GOES HERE */
    for item in numbers{
        mean += item as f64;
        if item < min{
            min = item;
        } else if item > max{
            max = item;
        }
    }

    mean /= numbers.len() as f64;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
