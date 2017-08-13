use std::collections::BTreeSet;
use std::cmp::{self, Ordering};

#[derive(Debug)]
pub struct IdPool {
    next_id: usize,
    free_list: BTreeSet<Range>,
}

impl IdPool {
    pub fn new() -> Self {
        IdPool {
            next_id: 0,
            free_list: BTreeSet::new(),
        }
    }

    pub fn new_id(&mut self) -> usize {
        if let Some(first_range) = self.free_list.iter().nth(0).cloned() {
            self.free_list.remove(&first_range);
            let reduced = first_range.pop_front();
            if !reduced.empty() {
                self.free_list.insert(reduced);
            }
            return first_range.min;
        }

        let id = self.next_id;
        self.next_id += 1;
        return id;
    }

    pub fn return_id(&mut self, id: usize) {
        if id + 1 == self.next_id {
            self.next_id -= 1;
        } else {
            self.set_free(id);
        }

        while self.collapse_next() {}
    }

    fn set_free(&mut self, id: usize) {
        let range = Range::id(id);
        if self.free_list.contains(&range) {
            return;
        }

        let range_front = if id > 0 { range.push_front() } else { range };
        let range_back = range.push_back();
        let combine_front = self.free_list.get(&range_front).cloned();
        let combine_back = self.free_list.get(&range_back).cloned();

        match (combine_front, combine_back) {
            (Some(front_range), Some(back_range)) => {
                let combined = front_range.merge(range).merge(back_range);

                self.free_list.remove(&front_range);
                self.free_list.remove(&back_range);
                self.free_list.insert(combined);
            }
            (Some(front_range), None) => {
                let combined = front_range.merge(range);

                self.free_list.remove(&front_range);
                self.free_list.insert(combined);
            }
            (None, Some(back_range)) => {
                let combined = back_range.merge(range);

                self.free_list.remove(&back_range);
                self.free_list.insert(combined);
            }
            (None, None) => {
                self.free_list.insert(range);
            }
        }
    }

    fn collapse_next(&mut self) -> bool {
        if let Some(last_range) = self.free_list.iter().rev().nth(0).cloned() {
            if last_range.max + 1 == self.next_id {
                self.free_list.remove(&last_range);
                self.next_id = last_range.min;
                return true;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn id(id: usize) -> Self {
        Range { min: id, max: id }
    }

    fn empty(self) -> bool {
        self.min > self.max
    }

    fn push_front(mut self) -> Self {
        self.min -= 1;
        self
    }

    fn push_back(mut self) -> Self {
        self.max += 1;
        self
    }

    fn pop_front(mut self) -> Self {
        self.min += 1;
        self
    }

    fn merge(self, other: Self) -> Self {
        Range {
            min: cmp::min(self.min, other.min),
            max: cmp::max(self.max, other.max),
        }
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Range {}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let srange = self.min...self.max;
        if srange.contains(other.min) || srange.contains(other.max) {
            return Ordering::Equal;
        }

        self.min.cmp(&other.min)
    }
}
