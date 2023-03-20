fn main() {
    let flowerbed: Vec<i32> = vec![1,0,0,0,0,1,0,0,1,0,0,0,0,0];
    let n = 1;
    // true
    
    // from 1 to 1, tally the 0s
    // 1,0 and 0,1 should not be tallied
    let mut tally_group: Vec<i32> = Vec::new();
    let mut group = 0;
    for (index,flower) in flowerbed.iter().enumerate() {
        if *flower == 0 {
            // Look behind and foward for a 1 value
            if index == 0 {
                group+=1;
                continue
            }
            if index == flowerbed.len()-1 {
                group+=1;
                tally_group.push(group);
                continue
            }
            let behind = flowerbed[index - 1];
            let ahead = flowerbed[index + 1];
            if ahead == 1 || behind == 1 {
                // If look ahead or behind is a one, do not add to group
                continue
            } else {
                // If a one is not detected, add to group
                group+=1;
            }
        } else {
            tally_group.push(group);
            group = 0;
        }
    }
    
    dbg!(tally_group);
}
