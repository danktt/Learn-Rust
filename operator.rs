use std::io::{self, Write};

fn main() {
    print!("Write a number in your multiplication table: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro to read a line");

    let numero: i32 = input.trim().parse().expect("Please, write a valid number");

    println!("multiplication table of: {}: ", numero);

    for i in 0..=10 {
        let resultado = numero * i;
        println!("{} x {} = {}", numero, i, resultado);
    }
}

