use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        bin2dec(&input);
    }
}

fn bin2dec(input: &str) {
    let final_value: usize = match usize::from_str_radix(input.trim(), 2) {
        Ok(num) => num,
        Err(_) => {
            println!("The input contains non-binary values");
            return;
        }
    };
    println!("Result: {}", final_value);
}
