use std::io;

fn to_float(input: &str) -> f64{
    let x: f64 = input.trim().parse::<f64>().unwrap();
    x
}

fn main() {
    let mut nota_1: String = String::new();
    println!("Digite a sua primeira nota: ");
    io::stdin().read_line(&mut nota_1).expect("Falha ao ler o input");
    let nota_1: f64 = to_float(&nota_1);

    let mut nota_2: String = String::new();
    println!("Digite a sua segunda nota: ");
    io::stdin().read_line(&mut nota_2).expect("Falha ao ler o input");
    let nota_2: f64 = to_float(&nota_2);

    let mut nota_3: String = String::new();
    println!("Digite a sua terceira nota: ");
    io::stdin().read_line(&mut nota_3).expect("Falha ao ler o input");
    let nota_3: f64 = to_float(&nota_3);

    let mut nota_4: String = String::new();
    println!("Digite a sua quarta nota: ");
    io::stdin().read_line(&mut nota_4).expect("Falha ao ler o input");
    let nota_4: f64 = to_float(&nota_4);

    let media: f64 = (nota_1 + nota_2 + nota_3 + nota_4) / 4.0;

    if media >= 7.0{
        println!("Parabéns, você foi aprovado, sua média foi de: {}", media);
    } else if media < 7.0 && media >= 5.0 {
        println!("Você está de recuperação, sua média foi de: {}", media);
    } else {
        println!("Infelizmente você foi reprovado, sua média foi de: {}", media);
    }

}