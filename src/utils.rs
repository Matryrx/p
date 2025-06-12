use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn load_proxies() -> Vec<String> {
    let data = fs::read_to_string("proxies.txt").unwrap_or_default();
    data.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn get_random_proxy(proxies: &Vec<String>) -> Option<String> {
    let mut rng = thread_rng();
    proxies.choose(&mut rng).cloned()
}
