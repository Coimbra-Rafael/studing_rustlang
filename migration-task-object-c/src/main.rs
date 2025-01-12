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
}