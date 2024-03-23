use std::cmp;

pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    };
    println!("Entered find_maximum_length : {:?}", nums);
    // declare function : find_maximum_length_from_index(nums: Vec<i32>, biggest_index:u16)
    // example: [2, 6, 1, 1]
    // a. if nums[curr] < nums[prev], i.e. 1 < 6
    // b. if no more element left
    // we gonna add curr to prev which gaurantees increasing order
    // so return answer as min(1, nums.length - 1 (since we combine last 2 elems))
    // c. if elemens still left, add the next element to the current and
    // call the function with a new_nums as [nums(0..prev), nums[curr] + nums[curr+1], nums[curr+2..nums.length-1])
    // call find_maximum_length_from_index with new_nums and curr, i.e. [1,6,2], 2
    // d. else nums[curr] > nums[prev]
    // go to next iteration using curr + 1

    /*
     * Examples:
     * [2,4,2,7,2] -> [2,4,9,2] -> [2,4,11] -> answer: 3
     * [6,4,2,1] -> [6, 6, 1] -> [6, 7] -> answer: 2
     * [6,1, 1] -> [6, 2] -> [8]
     */
    return find_maximum_length_from_index(nums, 1);
}

// curr is pointer to where to start comparing elements
fn find_maximum_length_from_index(nums: Vec<i32>, curr: usize) -> i32 {
    let prev = curr - 1;
    println!(
        ">>>Entered find_maximum_length_from_index with prev: {}, curr: {}",
        prev as i32, curr as i32
    );
    // a.
    println!(
        "a. (nums[curr] < nums[prev] means {} < {}",
        nums[curr], nums[prev]
    );
    if nums[curr] < nums[prev] {
        if (curr + 1) == nums.len() {
            // b.
            println!(
                "b. no more elements. Returning answer as {}",
                cmp::max(1, nums.len() - 1)
            );
            return cmp::max(1, (nums.len() - 1).try_into().unwrap());
        } else {
            // c.
            let mut new_nums: Vec<i32> = nums[..curr].iter().cloned().collect();
            new_nums.push(nums[curr] + nums[curr + 1]);
            new_nums.extend_from_slice(&nums[curr + 2..]);
            println!(
                "c. call find_maximum_length_from_index with  new_nums: {:?} index:{}",
                new_nums, curr
            );
            return find_maximum_length_from_index(new_nums, curr);
        }
    } else {
        // d.
        println!(
            "d. call find_maximum_length_from_index with  new_nums: {:?} index:{}",
            nums,
            curr + 1
        );
        if (curr + 1) == nums.len() {
            return nums.len() as i32;
        }
        return find_maximum_length_from_index(nums, curr + 1);
    }
}
