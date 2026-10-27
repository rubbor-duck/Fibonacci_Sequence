use std::io;

fn main() {
    
    calculator_menu();

}



fn fibonacci_sequence_calc(){
    //Repeat cannot be more than 92, or else overflow
    loop {
        println!("--Welcome to the Fibonoacci Sequence Calculator--");
        println!();
        println!("How many numbers do you want (max value of 92): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let repeat: i64 = input.trim().parse().expect("Not a number");

        println!();

        if repeat > 92 {
            println!("That value is greater than 92, please try again");
            continue;
        }

    let mut sequence_table= vec![];
    let mut x1:i64 = 0;
    let mut x2:i64 = 1;
    let mut i:i64 = 0;

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
            //magenta as default
        print!(",\x1b[35m {}\x1b[0m", x);
        }
    }

    print!(" ||");

    break;
}
}


fn factorial_calc(){
    //Repeat cannot exceed 20, or it will overflow
    loop{
    println!("--Welcome to the Factorial Calculator--");
    println!();
    println!("What number do you want to calculate (max value of 20): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let repeat: i64 = input.trim().parse().expect("Not a number");

    println!();
    
    if repeat > 20{
        println!("That value is greater than 20, please try again");
        continue;
    }

    let mut answer:i64 = 1;
    let mut i = repeat;

    while i > 0{
        answer = i * answer;

        i-=1;
    }

    println!("\x1b[31m {}\x1b[0m! is equal to \x1b[32m{} \x1b[0m", repeat, answer);

    break;
    }
}

fn calculator_menu(){
    let mut menu_select = -1;

    while menu_select != 0 {
        
        println!();
        println!();
        println!("--Welcome to the Calculator--");
        println!("Please Select a Function");
        println!("0: Exit the program");
        println!("1: Fibonacci Sequence Calculator");
        println!("2: Factorial Calculator");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        menu_select = input.trim().parse().expect("Not a number");

        match menu_select{
            0=>println!("Have a good day!"),
            1=>fibonacci_sequence_calc(),
            2=>factorial_calc(),
            _=>println!("That is not a vaild option"),
        }
    }
}