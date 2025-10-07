fn main(){
    let number1: i32 = 5;
    let number2: i32 = 5;

    if number1 > number2{
        println!("{} > {}", number1, number2)
    } else {
        println!("{} <= {}", number1, number2)
    }

    let number3: i32 = 5;
    let number4: i32 = 10;
    let number5: i32 = 15;
    let mut _result: i32 = 0;

    if number3 > number4{
        _result = number1;
    } else if number4 > number5{
        _result = number4;
    } else {
        _result = number5;
    }

    println!("Result 1: {}", _result);

    let value1: i32 = 5;
    let value2: i32 = 10;
    let value3: i32 = 15;
    let value4: i32 = 20;
    let mut _result2: i32 = 0;

    if value1 < value2 && value1 < value3 && value1 > value4{
        _result2 = value1;
    } else if value2 < value3 && value2 > value4{
        _result2 = value2;
    } else if value3 > value4{
        _result2 = value3;
    }  else {
        _result2 = value4;
    }

    println!("Result 2: {}", _result2);

    let op1: i32 = 5;
    let op2: i32 = 10;
    let string1: &'static str = "I'm enjoying";
    let string2: &'static str = "the crab 🦀";
    let boolean1: bool = true;
    let boolean2: bool = false;
    let mut _result3: i32 = 0;

    if boolean1 && (op1 > op2 || string1 == "I'm enjoying"){
        _result3 = op1;
    } else if boolean2 || (op2 > op1 && string2 == "the crab 🦀"){
        _result3 = op2;
    } else {
        _result3 = op1 + op2;
    }

    println!("Result 3: {}", _result3);

}
