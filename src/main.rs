fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for value in v {
        sum += value
    }
    sum
}

fn borrow_ownership_sum(v: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for value in v {
        sum += *value
    }
    sum
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];

    //   the following will give error
    //    let sum = take_ownership_sum(values);
    //    let n_items = values.len();

    let sum = borrow_ownership_sum(&values);
    let n_items = values.len();

    println!("number of items={} sum={}", n_items, sum);
}
