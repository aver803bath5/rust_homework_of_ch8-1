fn main() {
    let mut list = vec![13, 49, 23, 19, 50, 13];
    list_vec(&list);

    let mean = get_mean(&list);
    let median = get_median(&mut list);
    let mode = get_mode(&list);

    // print!("This is a sorted list: ");
    // list_vec(&list); // This line of list has been sorted
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}

fn list_vec(vec: &Vec<i32>) {
    print!("List: ");
    for val in vec {
        print!("{}, ", val);
    }
    println!();
}

fn get_mean(list: &Vec<i32>) -> f32 {
    let length = list.len() as f32;
    let mut total = 0 as f32;
    let mean: f32;

    print!("Total: ");
    for (i, val) in list.iter().enumerate() {
        let f32_val = *val as f32;
        let f32_i = i as f32;

        if f32_i < length - 1.0 {
            print!("{} + ", f32_val);
        } else {
            print!("{} =", f32_val);
        }

        total += f32_val;
    }

    println!(" = {}", total);

    mean = total / length;
    mean
}

fn get_median(list: &mut Vec<i32>) -> i32 {
    let mid = list.len() / 2 - 1;
    list.sort();
    list[mid]
}

fn get_mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for val in list {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }

    let mut max: u32 = 0;
    let mut _max_key = 0;
    for (_key, val) in &map {
        if *val > max {
            max = *val;
            _max_key = **_key;
        }
    }

    _max_key
}
