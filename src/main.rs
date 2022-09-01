pub mod sort;

fn main() {
    println!("Hello, world!\n\n");
    let my_sort = sort::Sort {
        array: vec![5, 3, 1, 7, 3, 8, 6, 9, 2, 4],
    };
    let bsort = my_sort.bubble_sort(false);
    let bsort = my_sort.quick_sort(true);
    print!("{:?}", &bsort);

    println!("\n\n");
}
