// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(input: &String) {
    if input.ends_with("s") {
        println!("{} is plural", input);
    } else {
        println!("{} is singular", input);
    }
}

fn change(input: &mut String) {
    if !input.ends_with("s") {
        input.push_str("s");
    }
}

fn eat(input: String) -> bool {
    if input.starts_with("b") && input.contains("a") {
        true
    } else {
        false
    }
}

fn bedazzle(input: &mut String) {
    // *input = "sparkly".to_string();
    *input = String::from("sparkly");
}