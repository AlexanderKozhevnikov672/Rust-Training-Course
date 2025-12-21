// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    let mut first = None;
    let mut second = None;

    vec.iter().for_each(|x| {
        if first.is_none() {
            first = Some(*x);
        } else if first.unwrap_or_default() < *x {
            (first, second) = (Some(*x), first);
        } else if first.unwrap_or_default() != *x && second.unwrap_or(i32::MIN) <= *x {
            second = Some(*x);
        }
    });

    second
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    let n = init_sequence.len();
    let mut max_len = vec![1; n];
    let mut prev_pos = vec![None; n];

    for i in 1..n {
        for j in 0..i {
            if init_sequence[j] < init_sequence[i] && max_len[j] + 1 > max_len[i] {
                max_len[i] = max_len[j] + 1;
                prev_pos[i] = Some(j);
            }
        }
    }

    let mut result = Vec::new();
    let mut pos = (0..n).max_by_key(|i| max_len[*i]);

    while pos.is_some() {
        result.push(init_sequence[pos.unwrap_or_default()]);
        pos = prev_pos[pos.unwrap_or_default()];
    }

    result.into_iter().rev().collect()
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let mut last_space = true;
    sentence
        .trim()
        .chars()
        .map(|c| -> String {
            if c.is_whitespace() && last_space {
                "".to_string()
            } else if c.is_whitespace() {
                last_space = true;
                c.to_string()
            } else if last_space {
                last_space = false;
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut chars = HashSet::new();
    s.chars().all(|c| chars.insert(c.to_lowercase().to_string()))
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut num_count = HashMap::new();
    nums.into_iter().for_each(|x| *num_count.entry(x).or_insert(0) += 1);

    let mut data: Vec<(i32, i32)> = num_count.into_iter().collect();
    data.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    data.into_iter().take(k).map(|(num, _)| num).collect()
}
