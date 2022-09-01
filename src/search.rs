pub struct Search {
    pub array: Vec<i32>,
}

impl Search {
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

    // fn BinarySearch() {}
    // fn Dijkstra() {}
    // fn Hash() {}
}
