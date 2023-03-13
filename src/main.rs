fn find_greatest_num(spread: &Vec<i32>) -> i32 {
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

fn sum(spread: &Vec<i128>) -> i128 {
    let mut total = 0;
    for value in spread {
        total += value;
    }
    return total;
}

fn speed_test(piles: &Vec<i32>, test_k: i128, target_hours: i128) -> &'static str {
    let mut hours_per_pile: Vec<i128> = Vec::new();

    for pile in piles {
        let time = (*pile as f32) / (test_k as f32);
        hours_per_pile.push(time.ceil() as i128);
    }

    let hours = sum(&hours_per_pile);

    let result = match hours {
        hours if hours == target_hours => "even",
        hours if hours > target_hours => "slow",
        hours if hours < target_hours => "fast",
        _ => "not sure",
    };


    return result;
}

fn search(piles: Vec<i32>, target_hours: i128) -> i128 {
    let mut k: i128 = 1;
    let mut even_values: Vec<i128> = Vec::new();
    let mut fast_value: i128 = 0;

    loop {
        dbg!(k);
        let speed = speed_test(&piles, k, target_hours); 
        if speed == "even" {
            if even_values.len() == 1 {
                break;
            }
            even_values.push(k);
            k+=1;
            continue;
        }else if speed == "fast" {
            fast_value = k;
            break;
        } else {
            k+=1;
        }

    }
    if even_values.len() == 0 {
        fast_value as i128
    } else {
        even_values[0] as i128
    }
}

fn main() {
    let piles: Vec<i32> = vec![332484035,524908576,855865114,632922376,222257295,690155293,112677673,679580077,337406589,290818316,877337160,901728858,679284947,688210097,692137887,718203285,629455728,941802184];
    let h = 823855818;
    // k = 4;

    let result = search(piles, h);

    dbg!(result);

}
