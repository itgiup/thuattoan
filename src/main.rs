pub mod search;
pub mod sort;

use rand;
use std::time::{Duration, Instant};

fn random_array(len: i32) -> Vec<i32> {
    let mut array = Vec::new();
    (0..len).for_each(|_| {
        array.push(rand::random::<i32>());
    });
    return array;
}
/*
fn main1() {
    println!("Hello, world!\n\n");
    let my_sort = sort::Sort {
        // array: vec![5, 3, 1, 7, 3, 8, 6, 9, 2, 4],
        array: random_array(1_000),
    };
    let array = my_sort.get_array();

    println!("quick sort:");
    let qsort = my_sort.quick_sort(array, false);

    println!("\n\nbubble sort:");
    let bsort = my_sort.bubble_sort(false);

    println!(
        "\n|\nV\n\n
        array:
        {:?}

        quick sort:
        {:?}

        bubble sort:
        {:?}",
        my_sort.array, qsort, bsort
    );

    println!("\n\n");
}
 */
fn main() {
    let array_len = 5_000_000;
    let index = 4_850_000;

    let my_search = search::Search::new(random_array(array_len));

    let start_binary = Instant::now();
    let binary_search_result = my_search
        .binary(0, my_search.array.len() - 1, my_search.array[index])
        .unwrap();
    let binary_search_duration = start_binary.elapsed();
    let start_linear = Instant::now();
    let linear_search_result = my_search.linear(my_search.array[1500000]);
    let linear_search_duration = start_linear.elapsed();

    println!(
        "\n\n
        value: {}
        linear_search_result {:?},
        binary_search_result {:?},
        linear_search_duration {:?}
        binary_search_duration {:?},
        ",
        my_search.array[index],
        linear_search_result,
        binary_search_result,
        linear_search_duration,
        binary_search_duration,
    );
}
