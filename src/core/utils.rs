use std::collections::{HashMap, HashSet};

pub fn retain_alive<T>(current: &mut HashMap<i32, T>, alive: &HashSet<i32>) -> Vec<T> {
    let keys_to_remove: Vec<i32> = current
        .keys()
        .filter(|key| !alive.contains(key))
        .copied()
        .collect();

    keys_to_remove
        .into_iter()
        .filter_map(|key| current.remove(&key))
        .collect()

    // let to_remove: Vec<T> = current
    //     .iter()
    //     .filter(|(key, _)| !alive.contains(key))
    //     .map(|(_, v)| *v)
    //     .collect();
    // current.retain(|key, _| alive.contains(key));
    // to_remove
}
