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
pub(crate) fn retain_alive<T>(current: &mut HashMap<i32, T>, alive: &HashSet<i32>) -> Vec<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: build a HashMap<i32, &str> from a slice of (key, value) pairs.
    fn make_map(entries: &[(i32, &'static str)]) -> HashMap<i32, &'static str> {
        entries.iter().copied().collect()
    }

    // Helper: build a HashSet<i32> from a slice.
    fn make_set(ids: &[i32]) -> HashSet<i32> {
        ids.iter().copied().collect()
    }

    #[test]
    fn all_alive_removes_nothing() {
        let mut current = make_map(&[(1, "a"), (2, "b"), (3, "c")]);
        let alive = make_set(&[1, 2, 3]);
        let removed = retain_alive(&mut current, &alive);
        assert!(
            removed.is_empty(),
            "nothing should be removed when all keys are alive"
        );
        assert_eq!(current.len(), 3);
    }

    #[test]
    fn none_alive_removes_all() {
        let mut current = make_map(&[(1, "a"), (2, "b"), (3, "c")]);
        let alive = make_set(&[]);
        let mut removed = retain_alive(&mut current, &alive);
        removed.sort();
        assert_eq!(removed, vec!["a", "b", "c"]);
        assert!(current.is_empty(), "all entries should be removed");
    }

    #[test]
    fn partial_alive_removes_only_dead() {
        let mut current = make_map(&[(1, "a"), (2, "b"), (3, "c")]);
        let alive = make_set(&[1, 3]);
        let removed = retain_alive(&mut current, &alive);
        // Only key 2 should be removed.
        assert_eq!(removed, vec!["b"]);
        assert!(current.contains_key(&1));
        assert!(!current.contains_key(&2));
        assert!(current.contains_key(&3));
    }

    #[test]
    fn empty_map_returns_empty_vec() {
        let mut current: HashMap<i32, &str> = HashMap::new();
        let alive = make_set(&[1, 2, 3]);
        let removed = retain_alive(&mut current, &alive);
        assert!(removed.is_empty());
        assert!(current.is_empty());
    }

    #[test]
    fn alive_set_with_keys_not_in_map_is_harmless() {
        // Keys in `alive` that don't exist in `current` should be silently ignored.
        let mut current = make_map(&[(1, "a")]);
        let alive = make_set(&[1, 99, 100]);
        let removed = retain_alive(&mut current, &alive);
        assert!(removed.is_empty());
        assert_eq!(current.len(), 1);
    }

    #[test]
    fn single_entry_removed_when_not_alive() {
        let mut current = make_map(&[(42, "only")]);
        let alive = make_set(&[]);
        let removed = retain_alive(&mut current, &alive);
        assert_eq!(removed, vec!["only"]);
        assert!(current.is_empty());
    }

    #[test]
    fn single_entry_kept_when_alive() {
        let mut current = make_map(&[(42, "only")]);
        let alive = make_set(&[42]);
        let removed = retain_alive(&mut current, &alive);
        assert!(removed.is_empty());
        assert!(current.contains_key(&42));
    }

    #[test]
    fn removed_values_are_the_correct_ones() {
        // Verify the actual values (not just count) that come back are the ones
        // whose keys were absent from alive.
        let mut current = make_map(&[(10, "a"), (20, "b"), (30, "c")]);
        let alive = make_set(&[20]);
        let mut removed = retain_alive(&mut current, &alive);
        removed.sort();
        assert_eq!(removed, vec!["a", "c"]);
        // Only key 20 should remain.
        assert_eq!(current.len(), 1);
        assert_eq!(current[&20], "b");
    }
}
