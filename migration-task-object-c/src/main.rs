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
}