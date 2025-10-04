use std::io;

fn to_integer(input: &str) -> i32 {
    let x: i32 = input.trim().parse::<i32>().unwrap();
    x
}

//--

fn main() {

    let mut input_fatorial: String = String::new();
    println!("Digite o número para calcular o fatorial: ");
    io::stdin().read_line(&mut input_fatorial).expect("Falha ao ler o input");
    let mut _input_fatorial_int: i32 = to_integer(&input_fatorial);
    let mut _fatorial: i32 = 1;

    //--

    while _input_fatorial_int != 1 {
        _fatorial *= _input_fatorial_int;
        _input_fatorial_int = _input_fatorial_int - 1;
    }

    println!("O fatorial é: {}", _fatorial);
}
