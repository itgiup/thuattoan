pub mod search;
pub mod sort;

use rand;
use search::{Search, Node};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

fn _random_array(len: i32) -> Vec<i32> {
    let mut array = Vec::new();
    (0..len).for_each(|_| {
        array.push(rand::random::<i32>());
    });
    return array;
}

fn read_nodes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<search::Node>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let nodes:Vec<Node> = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(nodes)
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
    println!("\n\n");
    /*
    let array_len = 5_000_000;
    let index = 4_000_000;

    let numbers_random = random_array(array_len);
        let start_quick_sort = Instant::now();
        let array_sorted = &(sort::Sort::quick(numbers_random.to_vec(), false));
        println!(
            "time_quick_sort: {:?} {}\n",
            start_quick_sort.elapsed(),
            Search::check_sorted(array_sorted.to_owned())
        );
    */
    /*
    let start_bubble_sort = Instant::now();
    let array_bubble_sorted = &(sort::Sort::bubble(numbers_random, false));
    println!(
        "time_bubble_sort: {:?} {}\n",
        start_bubble_sort.elapsed(),
        Search::check_sorted(array_bubble_sorted.to_owned())
    );
    */
    // println!("array: {:?}\n", numbers_random);
    /*
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
        "\n
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
    );*/

    let nodes = read_nodes_from_file("graph_nodes.json") .unwrap();
    println!("{:#?}", nodes);
    // for x in nodes {
    //     println!("{:?} ", x);
    //     break;
    // }
}
