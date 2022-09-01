pub struct Sort {
    pub array: Vec<i32>,
}

impl Sort {
    pub fn copy(&self) -> Vec<i32> {
        let new_array = &self.array.clone();

        return new_array.to_vec();
    }

    pub fn quick_sort(&self, print: bool) -> Vec<i32> {
        let mut array = self.copy();

        return array;
    }

    pub fn bubble_sort(&self, print: bool) -> Vec<i32> {
        let mut array = self.copy();

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
