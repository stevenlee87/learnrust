use std::collections::HashMap;

fn main() {
    let mut nums = vec![2, 5, 3, 4, 8, 6, 7, 9, 1];
    // let mut nums = vec![2, 5, 3, 4, 5, 8, 6, 7, 9, 1];

    let aver = average(&nums);
    println!("nums'average is {}", aver);

    let mid = middle(&mut nums);
    println!("nums'middle is {}", mid);

    let test_mode = vec![1, 2, 2, 3, 3, 3];
    // let test_mode = vec![1, 2, 3];
    let m = mode(&test_mode);
    println!("nums'mode is {:?}", m);
}

// average 求一系列数的平均数
fn average(nums: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    let length = nums.len();
    for value in nums {
        sum += value;
    }

    let result = sum as f64 / length as f64;
    result
}

// 可以通过把所有观察值高低排序后
// 找出正中间的一个作为中位数。
// 如果观察值有偶数个，
// 通常取最中间的两个数值的平均数作为中位数。
// middle 求一系列数的中位数
fn middle(nums: &mut Vec<i32>) -> f64 {
    nums.sort();
    let length = nums.len();

    if length % 2 == 1 {
        //nums中的数的总数是奇数
        //中间数即为中位数
        let center = length / 2; //索引从零开始
                                 // println!("middle:{}",center);
                                 // println!("{:?}",nums);
        return nums[center] as f64;
    } else {
        let center1 = length / 2 - 1;
        let center2 = length / 2;
        let result = (nums[center1] + nums[center2]) as f64 / 2.0;
        return result;
    }
}

// 一组数据中出现次数最多的数值，叫众数
// 如果有两个或两个以上个数出现次数都是最多的，
// 那么这几个数都是这组数据的众数。
// mode 求一系列数的众数
fn mode(nums: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    // 先用map统计所有数的个数
    for value in nums {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    // 取出个数最多的数
    for (num, count) in map {
        // println!("update:({},{})", *num, count);
        if max_count < count {
            result = Vec::new();
            result.push(*num);
            max_count = count;
        } else if max_count == count {
            result.push(*num);
        }
    }

    result
}
