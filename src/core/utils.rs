use std::collections::{HashMap, HashSet};

/// Retains only the entries in `current` whose keys are present in `alive`,
/// removing and returning all others.
///
/// This is used during TUIO frame processing to determine which tracked entities
/// have disappeared: any session ID no longer listed in the `alive` set is
/// removed from the active map and returned as a list of "removed" values.
///
/// # Arguments
///
/// * `current` - The map of currently tracked entities, keyed by session ID.
/// * `alive` - The set of session IDs that are still active in the latest frame.
///
/// # Returns
///
/// A [`Vec`] containing the values that were removed from `current` because
/// their keys were absent from `alive`.
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
}
