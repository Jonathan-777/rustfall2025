use std::io;

    const FREEZING_POINT_OF_WATER : f64 = 32.0;


//Assignment 1: Temperature Converter
fn fahrenheit_to_celsius(f: f64)-> f64{
    //subtract 32 from the Fahrenheit temperature,
    //then multiply the result by 5 and divide by 9
    let c = ((f - FREEZING_POINT_OF_WATER) * 5.0) / 9.0;
    return c;

}
fn celsius_to_fahrenheit(c: f64) -> f64{
    //To convert Celsius to Fahrenheit, use the formula: °F = (°C × 1.8) + 32.
    let f = (c * 1.8) + FREEZING_POINT_OF_WATER;
    return f;
}
fn assignment1(){
   
    // printing fahrenheit to celsius
    println!("\t\tprinting fahrenheit to celsius:\n");
    let  fahrenheit_var = 88.0;
    let mut celsius_temp = fahrenheit_to_celsius(fahrenheit_var);
    println!("{} degrees in fahrenheit are equal to  {:.1} in celsius",fahrenheit_var , celsius_temp);
    for i in 89..94{
        celsius_temp = fahrenheit_to_celsius(i as f64);
        println!("{} degrees in fahrenheit are equal to  {:.1} in celsius ",i , celsius_temp);

    }
    // printing celsius to fahrenheit
    println!("\n\t\t printing celsius to fahrenheit:\n");
    let celsius_var = 77.0;
    let mut fahrenheit_temp = celsius_to_fahrenheit(celsius_var);
    println!("{} degrees in celsius are euqal to {:.1} in fahrenheit",celsius_var, fahrenheit_temp);
    for i in 78..83{
        fahrenheit_temp = celsius_to_fahrenheit(i as f64);
        println!("{} degrees in celsius are euqal to {:.1} in fahrenheit", i , fahrenheit_temp)
    }


    



}
// Assignment 2: Number Analyzer
fn is_even(n: i32) -> bool{
    n % 2 == 0 //  as discussed in class, no return statement needed since its the last line
}
fn assignment2(){
    let numbers: [i32; 10] = [9,23,15,30,42,21,41,64,18,33];

    for i in 0..numbers.len(){
        if(numbers[i] % 3) == 0 && (numbers[i] % 5) == 0{
            println!("FizzBuzz");
        }else if numbers[i] % 3 == 0{
            println!("Fizz");
        }else if numbers[i] % 5 == 0{
            println!("Buzz");
        }else if !is_even(numbers[i]){
            println!("{} is NOT Even",numbers[i]);
        }else if is_even(numbers[i]){
            println!("{} is Even!",numbers[i]);
        }
    }
    let mut sum = 0;
    let mut largest = i32::MIN;
    for i in 0..numbers.len(){
        sum += numbers[i];
      if numbers[i] > largest {
        largest = numbers[i];
    }

    }
    println!("\n\t\t the sum of all numbers in the array is : {}",sum);
    println!("\n\t\t The largest number in the array is: {}",largest);


}
// Assignment 3: Guessing Game
fn check_guess(guess: i32, secret:i32)->i32{
    if guess > secret{
        return 1;
    }else if guess < secret{
        return -1;
    }else if guess == secret{
        return 0;
    }else{
        eprintln!("Unexpected case in check_guess!");
        return -999; // sentinel error code
    }

}
fn assignment3(){
    let mut guesses_count:i32 = 0;
    let mut current_guess:i32 = 98;
    let  secret_num:i32 = 532;

        let mut guess :i32 = check_guess(current_guess, secret_num);
        guesses_count += 1;
        println!("This is a integer guessing program we automatically guessed {}, type your guess: ",current_guess);
        if guess == 0{
            println!("You guessed the right number in {} guesses! congratulations",guesses_count);
            return ;
        }else if guess > 0{
            println!("\nYour first guess {} is too high!\n", current_guess);
        }else if guess < 0{
            println!("\nYour first guess {} is too low!\n", current_guess);
        }

    loop{

        

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let temp = input.trim();
       match temp.parse::<i32>() {
            Ok(number) => {
                current_guess = number;
            }
            Err(_) => {
                println!("Your input is not a valid i32 number, please try again!\n");
                continue;
            }
        }
       guesses_count += 1;
        guess = check_guess(current_guess, secret_num);
        if guess == 0{
            println!("You guessed the right number in {} guesses! congratulations",guesses_count);
            break;
        }else if guess > 0{
            println!("Your guess is too high!\n");
        }else if guess < 0{
            println!("Your guess is too low!\n");
        }
        

    }


}

fn main() {
    println!("\nAssignment 1: Temperature Converter");
    assignment1();
    println!( "\nAssignment 2: Number Analyzer");
    assignment2();
    println!("\nAssignment 3: Guessing Game");
    assignment3();

}

