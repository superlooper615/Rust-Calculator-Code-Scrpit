

fn main() {
    // fs::remove_file("/home/user/rustProject/my_calculator/src/calculator.rs")?;

    // let  mut file = File::create("/home/user/rustProject/my_calculator/src/calculator.rs")?;

    // write!(file,"fn calculator(number1: i32, number2: i32){{");
    // for number1 in 1..4 {
    //     for number2 in 1..4 {
    //         write!(file, "if number1 == {} && number2 == {} {{"
    //         , number1, number2);

    //         write!(file,"\tprintln!(\" \\t{} x {} is: {}\")", number1, number2, number1* number2);
    //         // println!("{} x {} = {}",
    //         // number1, number2, number1 * number2);
    //     }
    // }
    // write!(file,"}}");
    // Ok(())

    // script_generator(); //Create calculator source code


    // println!("The secret number is: {}", secret_number);

    // loop {

    //     println!("Please input your calculation:");

    //     let mut guess = String::new();
        
    //     io::stdin().read_line(&mut guess)
    //         .expect("Failed to read line");

    //     if guess.trim() == "Q"{
    //         println!("you win!");
    //     }



    // }

    script_generator();

}

fn script_generator () {

    println!("use std::io;");

    println!("fn main() {{");

    println!("\tloop{{ \n
        \t println!(\"Please input your calculation:\"); \n
        \t let mut input = String::new(); \n
        \t io::stdin().read_line(&mut input) \n
        \t \t .expect(\"Failed to read line\"); \n
        ");

    calculation_generator();

    println!("\t}}");

    println!("}}");

}

fn calculation_generator() {

    // let  mut file = File::create("/home/user/rustProject/my_calculator/src/calculator.rs")?;
    
    // write!(file, "fn calculator(number1: i32, number2: i32){{");

    // println!("\t \tfn calculator(number1: i32, number2: i32){{");

    let max_number = 40;
    let min_number = 1;

    println!("//=====Multiplication----");

    for number1 in min_number..max_number {
        for number2 in min_number..max_number {

            println!("\t \tif input.trim() == \"{}x{}\"{{"
                , number1, number2);

            println!("\t \t \tprintln!(\" \\t{} x {} = {}\")"
            , number1, number2, number1 * number2);

            println!("\t \t }}");
        
        }
    }

    println!("//=====Division----");

    for number1 in min_number..max_number {
        for number2 in min_number..max_number {

            println!("\t \tif input.trim() == \"{}/{}\"{{"
                , number1, number2);

            println!("\t \t \tprintln!(\" \\t{} ÷ {} = {}\")"
            , number1, number2, number1 / number2);

            println!("\t \t }}");
        
        }
    }

    println!("//=====Addition----");

    for number1 in min_number..max_number {
        for number2 in min_number..max_number {

             println!("\t \tif input.trim() == \"{}+{}\"{{"
             , number1, number2);

            println!("\t \t \tprintln!(\" \\t{} + {} = {}\")"
            , number1, number2, number1 + number2);

            println!("\t \t }}");
        
        }
    }

    println!("//=====Subtraction----");

    for number1 in min_number..max_number {
        for number2 in min_number..max_number {

             println!("\t \tif input.trim() == \"{}-{}\"{{"
            , number1, number2);

            println!("\t \t \tprintln!(\" \\t{} - {} = {}\")"
            , number1, number2, number1 - number2);

            println!("\t \t }}");
        
        }
    }
    // println!("\t\t}}");
}
