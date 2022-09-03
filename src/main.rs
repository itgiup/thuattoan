pub mod search;
pub mod sort;

use rand;
use search::Search;
use std::time::Instant;

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
    let array_len = 50_000;
    let index = 40_000;
    let numbers_random = random_array(array_len);
    let array_sorted = &(sort::Sort::quick(numbers_random.to_vec(), false));

    // println!("array: {:?}\n", numbers_random);
    // println!("array_quick_sort: {:?}\n", array_sorted);
    Search::check_sorted(array_sorted.to_owned());

    let start_linear = Instant::now();
    let linear_search_result =
        Search::linear(array_sorted.to_owned(), array_sorted[index]).unwrap();
    let linear_search_duration = start_linear.elapsed();

    let start_binary = Instant::now();
    let binary_search_result = Search::binary(
        array_sorted.to_owned(),
        0,
        array_sorted.len() - 1,
        array_sorted[index],
    )
    .unwrap();
    let binary_search_duration = start_binary.elapsed();

    println!(
        "\n\n
        value: {}
        linear_search_result:    {:?},
        binary_search_result:    {:?},

        linear_search_duration:  {:?}
        binary_search_duration:  {:?},
        ",
        array_sorted[index],
        linear_search_result,
        binary_search_result,
        linear_search_duration,
        binary_search_duration,
    );
}
