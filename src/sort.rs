// use rand::Rng;
#[derive(Copy, Clone)]
pub struct Sort {
}
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

impl Sort {
    /* quick_sort */
    pub fn quick(array: Vec<i32>, print: bool) -> Vec<i32> {
        if array.len() <= 1 {
            return array;
        }
        let pivot = array[array.len() - 1];

        let mut array_less: Vec<i32> = Vec::new();
        let mut array_greater: Vec<i32> = Vec::new();
        let mut array_sorted: Vec<i32> = Vec::new();
        for i in 0..(array.len() - 1) {
            if array[i] > pivot {
                array_greater.push(array[i]);
            } else {
                array_less.push(array[i]);
            }
        }
        if print {
            println!("{:?} {:} {:?}", array_less, pivot, array_greater);
        }
        // quick_sort(array_less);
        array_sorted.append(&mut Self::quick(array_less, print));
        array_sorted.push(pivot);
        array_sorted.append(&mut Self::quick(array_greater, print));

        return array_sorted;
    }

    /* bubble_sort */
    pub fn bubble(mut array: Vec<i32>, print: bool) -> Vec<i32> {
        let _array = array.clone();
        for i in 0..array.len() {
            let i_ = array.len() - i - 1;
            if print {
                println!("{} , ", i);
            }

            for j in 0..i_ {
                if print {
                    println!("---{} , ", j + 1);
                }
                if array[j] > array[j + 1] {
                    // let (a, b) = (array[j], array[j + 1]);
                    (array[j], array[j + 1]) = (array[j + 1], array[j]);
                    if print {
                        println!(">>> {:?} , ", array);
                    }
                }
            }
        }

        return array;
    }
}
