use std::io;

//TODO: Increase the vector table limit from 46 to a bigger number
//TODO: Add a way to print the sequence as a pyramid, or another way to increase code size.

fn main() {
    println!("--Welcome to the Fibonoacci Sequence Calculator--");
    println!();
    println!("How many numbers do you want (max value of 46): ");

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
        if (x % 2) == 0{
            //green if divisable by 2
            print!(",\x1b[32m {}\x1b[0m", x);
            continue;
        }

        else if (x % 3) == 0 {
            //blue if divisable by 3
            print!(",\x1b[34m {}\x1b[0m", x);
            continue;
        }
        
        else if (x % 5) == 0 {
            //red if divisable by 5
            print!(",\x1b[31m {}\x1b[0m", x);
            continue;
        }

        else {
            //magenta
        print!(",\x1b[35m {}\x1b[0m", x);
        }
    }

    print!(" ||");
}
