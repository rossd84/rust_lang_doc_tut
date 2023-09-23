fn main() {
    // calling fn w/o a return expression
    another_function(5, 'h');

    // calling fn with a return expression
    let x = five();
    let y = plus_one(x);
    // println!("The value of x is: {x}");
    // println!("The value of y is: {y}");

    if_statements();
    loops();
}

// function without a return expression
fn another_function(x: i32, unit_label: char) {
    println!("another_function() result: {x}{unit_label}");
}

// function with a return expression
fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    // if the line has no semicolon, it is read as a return expression
    x + 1
}

fn if_statements() {
    // if statements are the same as other languages
    // with one exception
    // they can be used on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("if_statements() result: {number}");
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels to disambiguate betwween multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
