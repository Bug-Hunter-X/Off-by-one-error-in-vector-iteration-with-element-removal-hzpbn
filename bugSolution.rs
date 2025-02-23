fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] % 2 == 0 {
            numbers.remove(i);
        } else {
            i += 1;
        }
    }
    println!("Result: {:?}", numbers);

    //Alternative solution using iterators for cleaner code
    let numbers2 = vec![1,2,3,4,5];
    let numbers3: Vec<i32> = numbers2.into_iter().filter(|&x| x % 2 != 0).collect();
    println!("Result (iterator): {:?}", numbers3);
} 