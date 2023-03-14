fn find_greatest_num(spread: &[i32]) -> i32 {
    let mut greatest_num = 0;

    for num in spread {
        let mut comparison_results: Vec<bool> = Vec::new();

        // Compare current num with the other numbers in the array.
        for compare_num in spread {
            if num > compare_num || num == compare_num {
                comparison_results.push(true);
            }
        }

        // If the comparison array is full of true values equal to the size of the original array
        // ... then the current number is the greatest.
        if &comparison_results.len() == &spread.len() {
            greatest_num = *num;
            break;
        }
    }
    return greatest_num;
}

fn speed_test(piles: &Vec<i32>, guess: i128, target_hours: i128) -> &'static str {
    let mut hours: i128 = 0;

    for pile in piles {
        let time = (*pile as f32) / (guess as f32);
        hours += time.ceil() as i128;
    }

    let result = match hours {
        hours if hours > target_hours => "slow",
        hours if hours <= target_hours => "fast",
        _ => "not sure",
    };


    return result;
}

fn get_range(max_k: i32) -> Vec<i32> {
    return (1..=max_k).collect();
}

fn guess(max_k: i32, min_k: i32, range: &Vec<i32>) -> i32 {
    let sum = max_k + min_k;
    let guess_index = (sum / 2) as usize;
    return range[guess_index];
}

fn main() {
    let piles: Vec<i32> = vec![30];
    let h = 29;
    // k = 4;
    
    let mut max_k = find_greatest_num(&piles);
    let mut min_k = 0;
    let range = get_range(max_k);
    let mut fast_guess: Vec<i32> = Vec::new();
    loop {
        let guess = guess(max_k, min_k, &range);
        let test = speed_test(&piles, guess.into(), h);

        if test == "slow" {
            min_k = guess
        } else {
            if fast_guess.len() > 0 {
                if fast_guess[0] > guess {
                    fast_guess[0] = guess;
                    max_k = guess;
                    continue
                } else {
                    break
                }
            }
            fast_guess.push(guess);
            max_k = guess;
        }
    }
    dbg!(fast_guess);
}
