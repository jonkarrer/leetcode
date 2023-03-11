fn sort(unsorted: &Vec<i32>) -> Vec<i32> {
    let mut sorted = unsorted.clone();

    for value in unsorted {
        let mut comparison_results: Vec<bool> = Vec::new();

        for comparison_value in unsorted {
            if value > comparison_value || value == comparison_value {
                comparison_results.push(true);
            }
        }

        let order = sorted.len() - comparison_results.len();
        sorted[order] = *value;
    }

    return sorted;
}

fn sum(spread: &Vec<i32>) -> i32 {
    let mut total = 0;
    for value in spread {
        total += value;
    }
    return total;
}

fn speed_test(piles: &Vec<i32>, bites: i32, target_hours: i32) -> &str {
    let mut hours_per_pile: Vec<i32> = Vec::new();

    for pile in piles {
        let time = (*pile as f32) / (bites as f32);
        hours_per_pile.push(time.ceil() as i32);
    }

    let hours = sum(&hours_per_pile);

    let result = match hours {
        hours if hours == target_hours => "even",
        hours if hours > target_hours => "slow",
        hours if hours < target_hours => "fast",
        _ => "not sure",
    };

    dbg!(hours_per_pile);

    return result;
}

fn range(ascending_piles: &Vec<i32>, h: i32) -> (i32, i32) {
    let mut min_number = 1;
    let mut max_number = ascending_piles[0];

    for num in ascending_piles {
        if num >= &min_number && num <= &max_number {
            let speed = speed_test(&ascending_piles, *num, h);

            if speed == "slow" {
                min_number = *num;
                break;
            } 
            if speed == "fast" {
                max_number = *num;
            }
        }
    }

    return (min_number, max_number);
}

fn main() {
    let piles = vec![30];
    let h = 29;
    let sorted_piles = sort(&piles);

    let (min_number, max_number) = range(&sorted_piles, h);

    let mut ascending_matching_values: Vec<i32> = vec![];
    let possible_values = (min_number..=max_number).collect::<Vec<i32>>();
    // for value in &possible_values {
    //     let test = speed_test(&sorted_piles, *value, h);

    //     if test == "even" {
    //         ascending_matching_values.push(*value);
    //     }
    // }

    // dbg!(min_number, max_number, even_number, range);
    // dbg!(ascending_matching_values, possible_values);
}
