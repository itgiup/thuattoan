pub mod sort;

fn main() {
    println!("Hello, world!\n\n");
    let my_sort = sort::Sort {
        array: vec![5, 3, 1, 7, 3, 8, 6, 9, 2, 4],
    };
    let mut array = my_sort.get_array();

    println!("quick sort:");
    let qsort = my_sort.quick_sort(array, true);

    let n: i32 = -10;
    println!("bubble sort:");
    let bsort = my_sort.bubble_sort(true);

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
