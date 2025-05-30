use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let x = 7;
    let y = fib(x);
    println!("The {x}th fibonacci number is {y}");
}

fn fib(n :u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }


// sol 2
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } 

    let mut a = 1;
    let mut b = 1;
    let mut c = 0;

    for i in (2..n) {
        c = a + b;
        a = b;
        b = c;
    }

    c

}
