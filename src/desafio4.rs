use std::io;

fn main() {
    println!("Desafio 4: Conversor de Temperatura");

    let mut temperatura = String::new();
    println!("inserir a temperatura em Graus Celsius pra Converte em Fahrenheit:");
    io::stdin()
    .read_line(&mut temperatura)
    .expect("erro ao receber a temperatura");


    let temperatura: f64 = temperatura.trim().parse().expect("Por favor, insira um número válido.");

    let calculo= (temperatura * 9.0/5.0)+32.0;
    
    println!("A temperatura de {:.2}°C corresponde a {:.2}°F.",temperatura, calculo);
}
