fn script_generator() {
    
    println!("Welcome to my calculator");

    println!("fn calculator(number1: i32, number2: i32){{");
    for number1 in 1..4 {
        for number2 in 1..4 {
            println!("if number1 == {} && number2 == {} {{"
            , number1, number2);

            println!("\tprintln!(\" \\t{} x {} is: {}\")", number1, number2, number1* number2);
            // println!("{} x {} = {}",
            // number1, number2, number1 * number2);
        }
    }
    println!("}}");
}