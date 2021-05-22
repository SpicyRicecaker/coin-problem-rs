use std::collections::HashMap;
use std::io::{self, Read};
#[derive(Hash, PartialEq, Eq, Debug)]
struct Coin {
    name: String,
    value: u8,
}
fn main() -> io::Result<()> {
    let mut running = true;
    let mut total_coins: Vec<Coin> = Vec::new();

    let mut buffer = String::new();
    while running {
        println!("Please enter a command");
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer[..].trim() {
                "help" => {
                    println!("`help` to print this list again. `exit|quit` to exit program. `coin` to add coin. `bal` to calculate amount of coins needed.")
                }
                "exit" | "quit" => {
                    println!("Thanks for using coin.");
                    running = false;
                }
                "coin" => {
                    total_coins.push(add_coin_stdin());
                }
                "bal" => {
                    let bal = get_bal();
                    let coins = coins(bal, &mut total_coins);
                    display(coins);
                }
                buffer => {
                    println!(
                        "Your command `{}` did not match any commands. Type `help` for help.",
                        buffer
                    )
                }
            },
            Err(e) => {
                println!("Please try again, some error occured {}", e)
            }
        }
        buffer.clear();
    }

    Ok(())
}

fn get_bal() -> f64 {
    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<f64>() {
                Ok(n) => {
                    if n >= 0_f64 {
                        return n;
                    } else {
                        println!("please enter a number > 0");
                    }
                }
                Err(e) => {
                    println!("error parsing float, sorry {}", e);
                }
            },
            Err(e) => {
                println!("Error reading input, {}", e);
            }
        }
        buffer.clear()
    }
}

fn add_coin_stdin() -> Coin {
    let mut buffer = String::new();

    let name: String;
    loop {
        println!("Please input the name of the coin");
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let clean_buffer = buffer.trim();
                if !clean_buffer.is_empty() {
                    name = clean_buffer.to_string();
                    break;
                } else {
                    println!("Name too short");
                }
            }
            Err(e) => {
                println!("Please try again, some error occured {}", e)
            }
        }
        buffer.clear()
    }

    buffer.clear();

    let value: u8;
    loop {
        println!("Please input the value of the coin in cents");
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let clean_buffer = buffer.trim();
                if !clean_buffer.is_empty() {
                    match clean_buffer.parse::<u8>() {
                        Ok(v) => {
                            value = v;
                            break;
                        }
                        Err(e) => {
                            println!(
                                "Error parsing {}, please enter a positive value in cents",
                                e
                            )
                        }
                    }
                } else {
                    println!("Value too short");
                }
            }
            Err(e) => {
                println!("Please try again, some error occured {}", e)
            }
        }
        buffer.clear()
    }

    Coin { name, value }
}

fn display(map: HashMap<&Coin, u32>) {
    for (coin, count) in map {
        println!("{} {}", coin.name, count)
    }
}

fn coins(input: f64, total_coins: &mut Vec<Coin>) -> HashMap<&Coin, u32> {
    total_coins.sort_by_key(|f| f.value);
    total_coins.reverse();

    let mut purse: HashMap<&Coin, u32> = HashMap::new();

    for coin in total_coins.iter() {
        purse.insert(coin, 0);
    }

    let mut remainder = input;
    loop {
        let mut accessed = false;
        for coin in total_coins.iter() {
            if remainder / (coin.value as f64 * 0.01) > 1_f64 {
                *purse.get_mut(coin).unwrap() += 1;
                remainder -= coin.value as f64 * 0.01;
                accessed = true;
                break;
            }
        }
        if !accessed {
            break;
        }
    }
    purse
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_44_cent() {
        let input = "0.44";
        let input: f64 = input.parse().unwrap();

        let mut total_coins = vec![
            Coin {
                name: "Penny".to_string(),
                value: 1,
            },
            Coin {
                name: "Nickle".to_string(),
                value: 5,
            },
            Coin {
                name: "Dime".to_string(),
                value: 10,
            },
            Coin {
                name: "Quarter".to_string(),
                value: 25,
            },
        ];

        let res = coins(input, &mut total_coins);

        res.iter().for_each(|(coin, count)| match &coin.name[..] {
            "Penny" => {
                dbg!(coin, count);
                assert_eq!(*count, 3);
            }
            "Nickle" => {
                dbg!(coin, count);
                assert_eq!(*count, 1);
            }
            "Dime" => {
                dbg!(coin, count);
                assert_eq!(*count, 1);
            }
            "Quarter" => {
                dbg!(coin, count);
                assert_eq!(*count, 1);
            }
            _ => {}
        });
    }
}
