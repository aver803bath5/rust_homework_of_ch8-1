mod statistics;

fn main() {
    let mut list = vec![13, 49, 23, 19, 50, 13];
    list_vec(&list);

    let mean = statistics::get_mean(&list);
    let median = statistics::get_median(&mut list);
    let mode = statistics::get_mode(&list);

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
