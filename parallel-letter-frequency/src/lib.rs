use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input : Vec<String> = input.iter().map(|str| str.to_string()).collect();
    let result = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for iw in 0..worker_count  {
        let input_clone = input.clone();
        let result_clone = result.clone();
        handles.push(thread::spawn(move || {
            let start = iw * input_clone.len() / worker_count;
            let end = if iw == worker_count - 1 {
                input_clone.len()
            } else {
                (iw + 1) * input_clone.len() / worker_count
            };

            for line in &input_clone[start..end] {
                let mut res = result_clone.lock().unwrap();
                for c in line.chars().filter(|c| c.is_alphabetic()) {
                    *res.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
                }
            }
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(result).expect("").into_inner().unwrap()
}
