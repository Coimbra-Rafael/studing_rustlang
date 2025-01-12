slint::slint! {
    import {Button, VerticalBox} from "std-widgets.slint";
    export component App inherits Window{
        in property <int> counter: 1;
        callback clicked <=> btn.clicked;
        VerticalBox {
            Text{ text: "Calculadora " + counter;}
            btn := Button { text: "Calcular";}
        }
    }
}

fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    app.on_clicked(move|| {
        let app = weak.upgrade().unwrap();
        app.set_counter(app.get_counter()+ 2);
    });
    app.run().unwrap();
}

//Abaixo se encontra o código da calculadora antiga

// use std::io;

// #[warn(unused_parens)]
// fn somar(x: f32, y: f32) -> f32 {
//     x + y
// }

// #[warn(unused_parens)]
// fn subtrair(x: f32, y: f32) -> f32 {
//     x - y
// }

// #[warn(unused_parens)]
// fn multiplicar(x: f32, y: f32) -> f32 {
//     x * y
// }

// #[warn(unused_parens)]
// fn dividir(x: f32, y: f32) -> f32 {
//     x / y
// }
// fn main() {
//     let mut n1 = String::new(); // Usar String para armazenar a entrada
//     let mut n2 = String::new(); // Usar String para armazenar a entrada

//     println!("Digite um número: ");
//     io::stdin().read_line(&mut n1).unwrap(); // Ler a linha e armazenar em n1

//     println!("Digite outro número: ");
//     io::stdin().read_line(&mut n2).unwrap(); // Ler a linha e armazenar em n2

//     // Converter as entradas para f32
//     let n1: f32 = n1.trim().parse().expect("Por favor, digite um número válido");
//     let n2: f32 = n2.trim().parse().expect("Por favor, digite um número válido");

//     println!("Segue os resultados abaixo: ");
//     println!("{}", somar(n1, n2));
//     println!("{}", subtrair(n1, n2));
//     println!("{}", multiplicar(n1, n2));
//     println!("{}", dividir(n1, n2));
// }
