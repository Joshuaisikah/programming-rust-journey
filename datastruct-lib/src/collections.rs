// Ch16 — Collections
//
// CONCEPTS:
//   HashMap<K, V>   — hash table; O(1) average insert/lookup
//   BTreeMap<K, V>  — sorted tree map; O(log n); keys must be Ord
//   HashSet<T>      — unique elements, unordered
//   VecDeque<T>     — double-ended queue; efficient push/pop at both ends
//   Entry API       — .entry(key).or_insert(val) for conditional insert

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

// ── HashMap patterns ──────────────────────────────────────────

/// Count how many times each element appears.
/// frequency(&[1, 2, 2, 3]) → {1: 1, 2: 2, 3: 1}
pub fn frequency<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> HashMap<T, usize> {
    todo!("use entry().and_modify().or_insert() pattern")
}

/// Group items by the result of `key_fn`.
/// group_by([1,2,3,4], |x| x % 2 == 0) → {true: [2,4], false: [1,3]}
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    todo!("entry(k).or_insert(vec![]).push(item)")
}

/// Swap keys and values. Panics if values are not unique.
/// invert_map({a: 1, b: 2}) → {1: a, 2: b}
pub fn invert_map<K, V>(map: HashMap<K, V>) -> HashMap<V, K>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    todo!("iterate, insert reversed pairs")
}

/// Merge two maps; when both have the same key, combine values with `f`.
pub fn merge_with<K, V, F>(mut a: HashMap<K, V>, b: HashMap<K, V>, f: F) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
    F: Fn(V, V) -> V,
{
    todo!("for each (k, v) in b: if key exists in a call f, else insert")
}

// ── BTreeMap patterns ─────────────────────────────────────────

/// Return the top `n` entries sorted by value (descending).
pub fn top_n<K: Clone, V: Ord + Clone>(
    map: &HashMap<K, V>,
    n: usize,
) -> Vec<(K, V)> {
    todo!("collect into vec, sort by value desc, take n")
}

/// Build a word-length index: length → sorted list of words with that length.
pub fn length_index(words: &[&str]) -> BTreeMap<usize, Vec<String>> {
    todo!("group words by word.len(); sort each group")
}

// ── Set operations ────────────────────────────────────────────

/// Return elements present in both sets.
pub fn intersect<T: Eq + std::hash::Hash + Clone>(
    a: &HashSet<T>,
    b: &HashSet<T>,
) -> HashSet<T> {
    todo!("a.iter().filter(|x| b.contains(x)).cloned().collect()")
}

/// Return elements in `a` that are not in `b`.
pub fn difference<T: Eq + std::hash::Hash + Clone>(
    a: &HashSet<T>,
    b: &HashSet<T>,
) -> HashSet<T> {
    todo!("a.iter().filter(|x| !b.contains(x)).cloned().collect()")
}

// ── VecDeque ──────────────────────────────────────────────────

/// Rotate a VecDeque left by `n` positions.
/// rotate_left([1,2,3,4,5], 2) → [3,4,5,1,2]
pub fn rotate_left<T>(mut deque: VecDeque<T>, n: usize) -> VecDeque<T> {
    todo!("use deque.rotate_left(n % deque.len())")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── frequency ─────────────────────────────────────────────

    #[test]
    #[ignore = "implement frequency"]
    fn test_frequency_basic() {
        let freq = frequency(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(freq[&1], 1);
        assert_eq!(freq[&2], 2);
        assert_eq!(freq[&3], 3);
    }

    #[test]
    #[ignore = "implement frequency"]
    fn test_frequency_empty() {
        let freq = frequency::<i32>(&[]);
        assert!(freq.is_empty());
    }

    #[test]
    #[ignore = "implement frequency"]
    fn test_frequency_single_element() {
        let freq = frequency(&['a', 'a', 'a']);
        assert_eq!(freq[&'a'], 3);
    }

    // ── group_by ──────────────────────────────────────────────

    #[test]
    #[ignore = "implement group_by"]
    fn test_group_by_parity() {
        let groups = group_by(vec![1, 2, 3, 4, 5], |x| x % 2 == 0);
        let mut evens = groups[&true].clone();
        let mut odds  = groups[&false].clone();
        evens.sort();
        odds.sort();
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds,  vec![1, 3, 5]);
    }

    // ── invert_map ────────────────────────────────────────────

    #[test]
    #[ignore = "implement invert_map"]
    fn test_invert_map_basic() {
        let mut m = HashMap::new();
        m.insert("a", 1_i32);
        m.insert("b", 2_i32);
        let inv = invert_map(m);
        assert_eq!(inv[&1], "a");
        assert_eq!(inv[&2], "b");
    }

    // ── merge_with ────────────────────────────────────────────

    #[test]
    #[ignore = "implement merge_with"]
    fn test_merge_with_sums_duplicates() {
        let mut a = HashMap::new();
        a.insert("x", 1_i32);
        a.insert("y", 2_i32);
        let mut b = HashMap::new();
        b.insert("x", 10_i32);
        b.insert("z", 3_i32);
        let merged = merge_with(a, b, |v1, v2| v1 + v2);
        assert_eq!(merged[&"x"], 11);
        assert_eq!(merged[&"y"], 2);
        assert_eq!(merged[&"z"], 3);
    }

    // ── length_index ──────────────────────────────────────────

    #[test]
    #[ignore = "implement length_index"]
    fn test_length_index_groups_by_length() {
        let idx = length_index(&["hi", "rust", "cat", "go", "code"]);
        assert_eq!(idx[&2], vec!["go", "hi"]);
        assert_eq!(idx[&3], vec!["cat"]);
        assert_eq!(idx[&4], vec!["code", "rust"]);
    }

    // ── set operations ────────────────────────────────────────

    #[test]
    #[ignore = "implement intersect"]
    fn test_intersect() {
        let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();
        let result = intersect(&a, &b);
        let mut v: Vec<i32> = result.into_iter().collect();
        v.sort();
        assert_eq!(v, vec![3, 4]);
    }

    #[test]
    #[ignore = "implement difference"]
    fn test_difference() {
        let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
        let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
        let result = difference(&a, &b);
        let mut v: Vec<i32> = result.into_iter().collect();
        v.sort();
        assert_eq!(v, vec![1, 2]);
    }

    // ── rotate_left ───────────────────────────────────────────

    #[test]
    #[ignore = "implement rotate_left"]
    fn test_rotate_left_basic() {
        let d: VecDeque<i32> = vec![1, 2, 3, 4, 5].into();
        let result: Vec<i32> = rotate_left(d, 2).into_iter().collect();
        assert_eq!(result, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    #[ignore = "implement rotate_left"]
    fn test_rotate_left_by_zero() {
        let d: VecDeque<i32> = vec![1, 2, 3].into();
        let result: Vec<i32> = rotate_left(d, 0).into_iter().collect();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
