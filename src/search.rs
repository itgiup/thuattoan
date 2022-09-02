pub struct Search {
    pub array: Vec<i32>,
}

impl Search {
    pub fn new(array: Vec<i32>) -> Self {
        Self { array }
    }

    pub fn linear(&self, x: i32) -> i32 {
        let mut i = 0;
        for v in &self.array {
            if v == &x {
                print!("{v} ");
                break;
            }
            i += 1;
        }
        return i;
    }

    /// .
    pub fn binary(&self, from: usize, to: usize, value: i32) -> Option<usize> {
        if to == from {
            if self.array[from] == value {
                return Some(from);
            } else {
                return None;
            }
        } else {
            let mid = (from + to) / 2;
            // println!("mid: {} {} {}", from, mid, to);
            let search_left = self.binary(from, mid, value);
            match search_left {
                Some(_) => {
                    // println!("search_left {:?} - from: {}", search_left, from);
                    return search_left;
                }
                None => {
                    // println!("search right - from: {} {}", mid + 1, to);
                    return self.binary(mid + 1, to, value);
                }
            }
        }
    }

    // fn Dijkstra() {}
    // fn Hash() {}
}
