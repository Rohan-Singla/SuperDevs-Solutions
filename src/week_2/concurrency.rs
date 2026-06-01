use std::thread;

fn process_values(values: Vec<i32>) -> Vec<i32> {

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(
        move || {
            for i in values {
                tx.send(i*2).unwrap();
            }
        }
    );

    return rx.iter().collect();


}

pub fn parallel_sum(nums: Vec<i32>) -> i32 {

    let mid = nums.len() / 2;

    let (left, right) = nums.split_at(mid);

    let left = left.to_vec();
    
    let right = right.to_vec();

    let h1 = thread::spawn(move || left.iter().sum::<i32>());
    let h2 = thread::spawn(move || right.iter().sum::<i32>());

    h1.join().unwrap() + h2.join().unwrap()
}