fn main() {
    let mut letters = ['a','b','c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter {}", first_letter);

    let numbers: [i32;5];
    // numbers = [0,0,0,0,0];
    numbers = [0;5];// same as above
    let index = numbers.len();
    println!("index 5 {}", numbers[index-1]);

    // multidimentional arrays
    let parking_lot = [[1,2,3],
                                     [4,5,6]];
    let num = parking_lot[1][2];
    println!("num {}", num);

    let _garage  =[[[0;100]; 20];5];

    // tuple
    let mut stuff:(u8,f32,char) = (10,3.14, 'x');
    stuff.0+=3;
    let first_item = stuff.0;
    println!("item one {}", first_item);
    let(_a,b,_c) = stuff;
    println!("b is {}", b);
}
