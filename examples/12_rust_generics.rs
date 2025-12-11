fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    let result = largest(&nums);

    println!("Largest = {} ", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest = {}", largest(&chars));
}
