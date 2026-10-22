use std::io;

fn main() {
    println!("--Welcome to the Fibonoacci Sequence Calculator--");
    println!();
    println!("How many numbers do you want: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let repeat: i32 = input.trim().parse().expect("Not a number");

    println!();

    fibonacci_sequence_calc(repeat);
}

fn fibonacci_sequence_calc(repeat: i32){
    let mut sequence_table= vec![];
    let mut x1 = 0;
    let mut x2 = 1;

    let mut i = 0;

    sequence_table.push(x2);

    while i < (repeat - 1){
        i+=1;

        let temp = x1 + x2;

        x1 = x2;

        x2 = temp;

        sequence_table.push(temp);
    }

    println!("Fibonacci Sequence for {} iterations", repeat);
    print!("|| 0");

    for x in sequence_table.iter(){
        print!(", {}", x);
    }

    print!(" ||");
}
