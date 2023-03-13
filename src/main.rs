fn speed_test(piles: &Vec<i32>, test_k: i128, target_hours: i128) -> &'static str {
    let mut hours: i128 = 0;

    for pile in piles {
        let time = (*pile as f32) / (test_k as f32);
        hours += time.ceil() as i128;
    }

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
