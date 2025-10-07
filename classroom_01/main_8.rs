/*Dado duas variáveis a = 15 e b = 40 , faça um programa para determinar o
* máximo divisor comum entre eles utilizando o laço While.​
*/

fn main(){
    let mut value_a: i32 = 15;
    let mut value_b: i32 = 40;

    while value_b != 0 {
        let temporaria: i32 = value_b;
        value_b = value_a % value_b;
        value_a = temporaria;
    }
    println!("O máximo divisor comum é: {}", value_a);
}