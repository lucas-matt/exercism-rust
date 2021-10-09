use std::collections::HashMap;
use std::thread::{spawn, JoinHandle};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut threads:Vec<JoinHandle<HashMap<char, usize>>> = Vec::new();

    for worker_num in 0..worker_count {
        let job:String = create_job(worker_num, worker_count, input);
        let thread = spawn_job(job);
        threads.push(thread)
    }

    gather(threads)
}

fn create_job(worker_num: usize, worker_count: usize, input:&[&str]) -> String {
    input.iter().enumerate()
        .filter(|(n, _)| n % worker_count == worker_num)
        .map(|(_, s)| s.to_string())
        .collect()
}

fn spawn_job(job: String) -> JoinHandle<HashMap<char, usize>> {
    spawn(move || {
        job.chars()
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
    })
}

fn gather(threads:Vec<JoinHandle<HashMap<char, usize>>>) -> HashMap<char, usize> {
    threads.into_iter()
        .fold(HashMap::new(), |mut acc, handle| {
            if let Ok(map) = handle.join() {
                for (char, count) in map.iter() {
                    *acc.entry(*char).or_insert(0) += count
                }
            }
            acc
        })
}