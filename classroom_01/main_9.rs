// FUNÇÕES 

fn doble(x: i32) -> i32{
    x * 2
}

fn max_number(a: i32, b: i32) -> i32{
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let nun: i32 = 5;
    let result = doble(nun);
    println!("O dobro de {} é {}", nun, result);
    let maior = max_number(10, 20);
    println!("O maior número entre 10 e 20 é {}", maior);
    println!("O maior número entre 30 e 15 é {}", max_number(30, 15));
    println!("O maior número entre -7 e 7 é {}", max_number(-7, 7));

}