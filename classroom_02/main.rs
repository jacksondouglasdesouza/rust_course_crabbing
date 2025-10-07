// TIPOS DE DADOS
// Rust é uma linguagem de tipagem estática, o que significa que o tipo de uma
// variável deve ser conhecido em tempo de compilação. No entanto, Rust também
// possui inferência de tipos, o que permite que o compilador deduza o tipo
// de uma variável com base no valor atribuído a ela.

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn main(){
// Exemplo de inferência de tipos
// U 32 é um tipo de dado que representa um número inteiro sem sinal de 32 bits - Unsigned 32 bits
// Ou seja, ele pode armazenar valores inteiros positivos de 0 a 4.294.967.295.
// Outro tipo de dado é o i32, que representa um número inteiro com sinal de 32 bits - Signed 32 bits
// Ele pode armazenar valores inteiros negativos e positivos, variando de -2.147.483.648 a 2.147.483.647.
let number: u32 = "45".parse::<u32>().expect("Nõ foi possível converter a string para um número");
let number_signed: i32 = -85;
println!("O número com sinal é: {}", number_signed);
println!("O número é: {}", number);

// OUTROS TIPOS DE DADOS

// - INTEIROS (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
// - PONTO FLUTUANTE (f32, f64)
// - BOOLEANS (bool)
// - CARACTERES (char) 'm' || 'M' || 'f' || 'F' || '1' || '$' || ' ' (espaço)
// - NUMBERS (i32, f64)
// - STRINGS (String, &str)
// Tuplas (tuples) e arrays (arrays)
/* - TUPLAS (tuples) - coleção de valores de diferentes tipos fixos */

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // Desestruturação da tupla
    println!("O valor de y é: {}", y);
    println!("O valor de z é: {}", z);
    println!("O valor de x é: {}", x);
    println!("O primeiro valor da tupla é: {}", tuple.0);
    println!("O segundo valor da tupla é: {}", tuple.1);
    println!("O terceiro valor da tupla é: {}", tuple.2);

    // - ARRAYS (arrays) - coleção de valores do mesmo tipo e tamanho fixos

    let array_integers: [i32; 5] = [100, 200, 300, 400, 500];
    println!("O tamanho do array é: {}", array_integers.len());
    println!("O primeiro valor do array é: {}", array_integers[0]);
    println!("O segundo valor do array é: {}", array_integers[1]);
    println!("O terceiro valor do array é: {}", array_integers[2]);
    println!("O quarto valor do array é: {}", array_integers[3]);
    println!("O quinto valor do array é: {}", array_integers[4]);
    // println!("O quinto valor do array é: {}", array_integers[5]); //index out of bounds: the length is 5 but the index is 5

    let array_floats: [f64; 3] = [1.111111, 2.2222, 3.333];

    println!("O tamanho do array é: {}", array_floats.len());
    println!("O primeiro valor do array é: {}", array_floats[0]);
    println!("O segundo valor do array é: {}", array_floats[1]);
    println!("O terceiro valor do array é: {}", array_floats[2]);

    let meses: [&str; 12] = [
        "Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
        "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
    ];

    println!("O tamanho do array é: {}", meses.len());
    println!("O primeiro mês do ano é: {}", meses[0]);
    println!("O segundo mês do ano é: {}", meses[1]);
    println!("O terceiro mês do ano é: {}", meses[2]);
    println!("O quarto mês do ano é: {}", meses[3]);
    println!("O quinto mês do ano é: {}", meses[4]);
    println!("O sexto mês do ano é: {}", meses[5]);
    println!("O sétimo mês do ano é: {}", meses[6]);
    println!("O oitavo mês do ano é: {}", meses[7]);
    println!("O nono mês do ano é: {}", meses[8]);
    println!("O décimo mês do ano é: {}", meses[9]);
    println!("O décimo primeiro mês do ano é: {}", meses[10]);
    println!("O décimo segundo mês do ano é: {}", meses[11]);

    // --

    let texto: &str = "Olá, mundo!";
    let mut texto_mutavel: String = String::from("Olá, mundo! Novamente!"); //

    println!(" ");
    println!("O texto é: {}", texto);
    println!("O texto mutável é: {}", texto_mutavel);
    texto_mutavel.push_str(" Adicionando mais texto.");
    println!("O texto mutável é: {}", texto_mutavel);

    // let texto_descartavel = "Texto de descartável"; //
    print!(" ");

    //--

    let numero1: i32 = 10021;
    let referencia_numero1: &i32 = &numero1; // referência é um ponteiro para o valor da variável.
    println!("O valor de numero1 é: {}", numero1);
    println!("O valor de referencia_numero1 é: {}", referencia_numero1);
    println!(" -- ");

    let mut numero1: i32 = 52;
    let referencia_numero1_mutacao: &mut i32 = &mut numero1;
    println!("O valor de referencia_numero2_mutacao é: {}", referencia_numero1_mutacao);

    println!(" -- ");
    println!("O valor de numero1 é: {}", numero1);

    //-- STANDARD --//
    // std:: is the standard library in Rust
    // io / Vec / &[T] / HashMap / HashSet / String / &str / Box / Rc / Arc / Cell /
    // RefCell / LinkedList / BinaryHeap / BTreeMap / BTreeSet / Option / Result
    // std::fmt / std::cmp / std::ops / std::iter / std::mem / std::ptr / std::slice / std::str
    // std::thread / std::time / std::fs / std::env / std::process / std::net e etc...

    //-- CASTING --//

    let inteiro:i32 = 1000;
    let inteiro_para_float: f64 = inteiro as f64; // casting de i32 para f64
    println!("O valor do inteiro é: {}", inteiro);
    println!("O valor do inteiro para float é: {}", inteiro_para_float);

    println!("O tipo de dado de inteiro é: {}", type_of(&inteiro));
    println!("O tipo de dado de inteiro_para_float é: {}", type_of(&inteiro_para_float));

    let numero2: i32 = 789;
    let numero2_para_string: String = numero2.to_string(); 
    println!("O valor de numero2 é: {}", numero2);
    println!("O valor de numero2_para_string é: {}", numero2_para_string);
    println!("O tipo de dado de numero2 é: {}", type_of(&numero2));
    println!("O tipo de dado de numero2_para_string é: {}", type_of(&numero2_para_string));


}
