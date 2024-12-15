use std::io;

fn main() {
    println!("Desafio 4: Conversor de Temperatura");

    let mut numero = String::new();
    println!("inserir um numero pra gera a tabuada do 1 a 10:");
   
    io::stdin()
    .read_line(&mut numero)
    .expect("erro ao receber o numero");

/*usa com while
let numero: i64 = numero.trim().parse().expect("Por favor, insira um número válido.");
*/
   
   
  // usa com for
  let numero: i32 = numero.trim().parse().expect("Por favor, insira um número válido.");

   
   let mut contador = 1;

  /*   com while
   println!("Tabuada do {}:", numero);
   while contador <= 10 {
        let resultado = numero * contador;
        println!("{} x {} = {}", numero, contador, resultado);
        contador += 1;
    }
    */

     
    println!("Tabuada do {}:", numero);
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }

}
