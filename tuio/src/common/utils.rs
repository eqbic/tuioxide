use std::collections::{HashMap, HashSet};

pub fn retain_alive<T>(current: &mut HashMap<i32, T>, alive: &HashSet<i32>) -> Vec<T>
where
    T: Copy,
{
    let to_remove: Vec<T> = current
        .iter()
        .filter(|(key, _)| !alive.contains(key))
        .map(|(_, v)| *v)
        .collect();
    current.retain(|key, _| alive.contains(key));
    to_remove
}
