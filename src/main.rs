pub mod search;
pub mod sort;

use rand;

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
    let my_search = search::Search {
        array: random_array(2000000),
    };
    println!(
        " \n\n{:} at {:}",
        //my_search.array,
        my_search.array[1500000],
        my_search.linear(my_search.array[1500000])
    );
}
