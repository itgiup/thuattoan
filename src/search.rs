pub struct Search {}
use serde::{Deserialize};

impl Search {
    pub fn check_sorted(array: Vec<i32>) -> bool {
        let mut i: usize = 0;
        for v in array.to_owned() {
            if (i < array.len() - 1) && (array[i + 1] < v) {
                println!("sorted: false {}", i);
                return false;
            }
            i += 1;
        }
        return true;
    }

    pub fn linear(array: Vec<i32>, x: i32) -> Option<usize> {
        let mut i: usize = 0;
        for v in array {
            if v == x {
                return Some(i);
            }
            i += 1;
        }
        return None;
    }

    /// .
    pub fn binary(array: Vec<i32>, from: usize, to: usize, value: i32) -> Option<usize> {
        let mid = (from + to) / 2;
        // println!("mid: {} {} {}", from, mid, to);

        if array[mid] == value {
            return Some(mid);
        }
        if array[mid] > value {
            // println!("search_left {:?} - from: {}", search_left, from);
            return Self::binary(array, from, mid, value);
        }
        if array[mid] < value {
            // println!("search right - from: {} {}", mid + 1, to);
            return Self::binary(array, mid + 1, to, value);
        }
        return None;
    }

    // fn Dijkstra() {}
    // fn Hash() {}
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NodePath {
    name: String,
    distance: f32,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    name: String,
    connects: Vec<NodePath>,
}

impl Node {
    
}
