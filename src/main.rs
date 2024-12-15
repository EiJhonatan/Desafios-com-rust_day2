use rand::Rng;
use std::io;

fn main() {
    println!("Desafio 6: Jogo de Adivinhação");

    // Gerar um número aleatório entre 1 e 100
    let numero_aleatorio = rand::thread_rng().gen_range(1..=100);

    let mut tentativas = 0;

    loop {
        // Incrementar o número de tentativas
        tentativas += 1;

        // Solicitar palpite do usuário
        let mut palpite = String::new();
        println!("Tente adivinhar um número entre 1 e 100:");
        io::stdin()
            .read_line(&mut palpite)
            .expect("Erro ao receber o palpite.");

        // Converter o palpite para número
        let palpite: i32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido!");
                continue;
            }
        };

        // Verificar o palpite
        if palpite < numero_aleatorio {
            println!("Seu palpite está muito baixo!");
        } else if palpite > numero_aleatorio {
            println!("Seu palpite está muito alto!");
        } else {
            println!(
                "Parabéns! Você acertou! O número era {}. Você precisou de {} tentativas.",
                numero_aleatorio, tentativas
            );
            break;
        }
    }
}
