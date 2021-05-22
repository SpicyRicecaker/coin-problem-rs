use std::f32::INFINITY;

// Greedy algorithm may not always give the best solution

fn main() {
    // let res = alex_lugo(8, &[1, 4, 5]);
    // println!("alex_lugo: {}", res);
    let res = coins(8, &[1, 4, 5]);
    println!("coins: {}", res);

    let res = coins(6, &[1, 3, 4]);
    println!("coins: {}", res);

    let res = coins(60, &[1, 10, 41]);
    println!("coins: {}", res);
}

// See https://www.youtube.com/watch?v=TecJkB-8kBI&t=186s
fn alex_lugo(amount: usize, coins: &[usize]) -> u32 {
    // All the possibilities to get 1cent, 2cent, etc. up to 22 cents
    let mut total_ways = vec![0; amount];
    // Different value coins that we have

    for i in 0..total_ways.len() {
        // ??? 0 indexed, first coin represents one cent
        let amount = i + 1;
        // If there's no way to reach the amount we want with our minimal valued coin...
        if amount < coins[0] {
            // Continue
            continue;
        }
        // If you want to give someone the five cent coin, and you have 5 cents, just give it to them
        if coins.contains(&amount) {
            total_ways[i] = 1;
            continue;
        }
        // Weird cases, > 1 coin
        let mut min = 1000000;

        for j in 0..coins.len() {
            // If the coin value is greater than the amount that we're trying for, don't even look at that coin
            if coins[j] > amount {
                break;
            }
            //
            let potent = total_ways[i - coins[j]] + 1;
            if potent < min {
                min = potent;
            }
            total_ways[i] = min;
        }
    }

    total_ways[total_ways.len() - 1]
}

// Try Khov explains this well @ https://trykv.medium.com/how-to-solve-minimum-coin-change-f96a758ccade
fn coins(amount: usize, coins: &[usize]) -> u32 {
    // Keep in mind coins must be sorted beforehand

    // Generate array of num of coins needed to arrive at value up to amount
    // Initiated to infinity
    let mut ways = vec![INFINITY; amount + 1];
    *ways.get_mut(0).unwrap() = 0_f32;

    // For every coin value
    for &coin in coins {
        // For every way
        for needed_amount in 0..ways.len() {
            // If the coin value is greater than the needed amount, we can't do anything
            // e.g. A nickle is greater than 3 cents
            if coin > needed_amount {
                continue;
            }
            // If the coin is equal to the current amount, replace past num of coins with 1
            // e.g. To get 5 cents, we just need 1 nickle
            if coin == needed_amount {
                *ways.get_mut(needed_amount).unwrap() = 1_f32;
                continue;
            }
            // Otherwise calculate the diff between our current_needed_amount and the value of our coin in question, and
            // use that diff to get the num of coins that we needed to get to that amount of coins
            // e.g. We stored up that we need 3 pennies to get to 0.03 cents. Now, with a nickle, to get to a value of 0.08, we need to get
            // 0.03, which we have stored up as 3 pennies to get to 0.03 cents
            let diff = needed_amount - coin;
            let amount_of_coins_needed_at_diff = *ways.get(diff).unwrap();

            let current = ways.get_mut(needed_amount).unwrap();

            // If our amount of coins needed to get diff + 1 is less than the current num of coins required to get it
            // e.g. our 1 nickle + 3 pennies to get 8 cents is less than our preset infinity num of coins
            if amount_of_coins_needed_at_diff + 1_f32 < *current {
                // Set current to past
                *current = amount_of_coins_needed_at_diff + 1_f32;
            }
        }
    }

    *ways.last().unwrap() as u32
}
