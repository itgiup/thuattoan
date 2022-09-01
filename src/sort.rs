// use rand::Rng;
// #[derive(Copy, Clone)]
pub struct Sort {
    pub array: Vec<i32>,
}
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

impl Sort {
    pub fn get_array(&self) -> Vec<i32> {
        let new_array = &self.array.clone();

        return new_array.to_vec();
    }

    // /* quick_sort */
    // pub fn partition(&self, mut array: Vec<i32>, low: i32, high: i32) -> i32 {
    //     let pivot = low.clone() as usize;
    //     let mut index = high.clone() as usize;
    //     for i in low..high {
    //         let ui = i as usize;
    //         if array[ui] < array[pivot] {
    //             (array[ui], array[index]) = (array[index], array[ui]);
    //             index += 1;
    //         }
    //     }
    //     (array[pivot], array[index]) = (array[pivot], array[index]);
    //     return index as i32;
    // }
    // pub fn random_pivot_partition(&self, mut array: Vec<i32>, low: i32, high: i32) -> i32 {
    //     let n = rand::thread_rng().gen_range(0..array.len()) as i32;
    //     let pvt = (low + n % (high - low + 1)) as usize;
    //     (array[high as usize], array[pvt]) = (array[pvt], array[high as usize]);
    //     return self.partition(array, low, high);
    // }
    // fn _quick_sort(&self, array: Vec<i32>, low: i32, high: i32) -> i32 {
    //     let pindex: i32;
    //     if low < high {
    //         pindex = self.random_pivot_partition(array, low, high);
    //         self._quick_sort(array, low, pindex - 1);
    //         self._quick_sort(array, pindex + 1, high);
    //     }
    //     return 0;
    // }
    // pub fn quick_sort(&self, print: bool) -> Vec<i32> {
    //     let array = self.get_array();
    //     println!("{:p} {:?}", &array[0], array);
    //     let len = array.len();
    //     // let pivot_i = rand::thread_rng().gen_range(0..len);
    //     self._quick_sort(array, 0, (len - 1) as i32);
    //     return Vec::new();
    // }

    /* quick_sort */
    pub fn quick_sort(&self, array: Vec<i32>, print: bool) -> Vec<i32> {
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
        println!("{:?}      {:}      {:?}", array_less, pivot, array_greater);
        // quick_sort(array_less);
        array_sorted.append(&mut self.quick_sort(array_less, print));
        array_sorted.push(pivot);
        array_sorted.append(&mut self.quick_sort(array_greater, print));

        // if print {
        //     println!(
        //         "{:} {:?}",
        //         pivot,
        //         array_sorted //, &array_greater, &array_less
        //     );
        // }
        return array_sorted;
    }

    /* bubble_sort */
    pub fn bubble_sort(&self, print: bool) -> Vec<i32> {
        let mut array = self.get_array();

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
