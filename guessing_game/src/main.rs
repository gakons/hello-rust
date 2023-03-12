use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Отгадай номер!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Пожалуйста, введи число.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Ты угадал, это число {guess}!");
                break;
        },
            Ordering::Greater => println!("Много."),
            Ordering::Less => println!("Мало."),
        }

    }
}
