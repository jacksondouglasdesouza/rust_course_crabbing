use std::io;

fn to_integer(data_input: &str) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    x
}


fn main() {

    let mut value1: String = String::new();
    println!("Insira um número: ");
    io::stdin().read_line(&mut value1).expect("ERROR - Falha ao ler o valor" );
    
    let mut value2: String = String::new();
    println!("Insira um número: ");
    io::stdin().read_line( &mut value2).expect("ERROR - Falha ao ler o valor" );

    //--

    if to_integer(&value1 ) > to_integer(&value2){
        println!("O valor {} é maior que o valor {}", value1, value2);
    } else if to_integer(&value1) < to_integer(&value2){
        println!("O Valor {} menor que o valor {}", value1, value2)
    } else {
        println!("O valor {} é igual ao valor {}", value1, value2);
    }

    //-- While Loop --//

    let mut sum: i32 = 0;
    let mut value3: String = String::new();

    println!("Insira um valor: ");
    io::stdin().read_line(&mut value3).expect("ERROR - Falha ao ler o valor");
    let mut value3: i32 = to_integer(&value3);

    while value3 != 0{
        let resto: i32 = value3 % 10;
        sum += resto;
        value3 /= 10;
    }

    print!("O valor da soma é: {}", sum);

}
