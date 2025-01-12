<<<<<<< HEAD
use std::io;

fn main() {
    println!("Digite a quantidade de macacos: ");

    let mut macacos = String::new();
    io::stdin().read_line(&mut macacos).expect("Falha ao ler a linha");
    match macacos.trim().parse::<i32>() {
        Ok(n) => {
            let bananas = match n {
                0..=2 => 4,
                3..=6 => 6,
                7..=14 => 10,
                15..=30 => 15,
                31..=60 => 25,
                _ => 0
            };
            println!("A quantidade de bananas necessária é: {}", bananas);
        },
        Err(_) => {
            println!("Digite um número válido");
        }
    }       
=======
use std::io::stdin;

fn main() {
    println!("Digite a quantidade de macacos: ");
    let mut macacos = String::new();
    let bananas: i32;
    stdin().read_line(&mut macacos).unwrap();

    bananas = match macacos.trim().parse::<i32>() {
        Ok(num) => match num {
            0..=2 => 4,
            3..=6 => 6,
            7..=14 => 10,
            15..=30 => 15,
            31..=60 => 25,
            _ => 0,
        },
        Err(_) => 0,
    };

    println!("Bananas: {}", bananas);
>>>>>>> c1084f5f5434b7c1f86bac0b00f55d3287f67131
}