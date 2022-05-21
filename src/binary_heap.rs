use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct BinaryHeap<'a, T: ?Sized> {
    heap: Vec<Item<'a, T>>,
    indices: HashMap<&'a T, usize>,
}

#[derive(Debug)]
pub struct Item<'a, T: ?Sized> {
    pub value: &'a T,
    pub weight: f64,
}

impl<'a, T: Eq + Hash + ?Sized > BinaryHeap<'a, T> {
    pub fn new() -> Self {
        BinaryHeap {
            heap: Vec::new(),
            indices: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: &'a T, weight: f64) {
        if self.indices.contains_key(value) {
            panic!("Value already present in the heap");
        }

        self.heap.push(Item{value, weight});
        let index = self.heap.len() - 1;
        self.indices.insert(value, index);

        self.percolate_up(index, weight)
    }

    pub fn peek(&self) -> Option<&Item<T>> {
        return self.heap.get(0);
    }

    pub fn pop(&mut self) -> Option<Item<T>> {
        if self.heap.len() == 0 {
            return None
        }

        self.swap(0, self.heap.len() - 1);
        return self.heap.pop().and_then(|item| {
            self.indices.remove(item.value);
            if self.heap.len() > 0 {
                self.percolate_down(0, self.heap[0].weight);
            }

            return Some(item);
        });
    }

    pub fn update_weight(&mut self, value: &T, new_weight: f64) {
        let index = *self.indices.get(value).expect("Value not in heap");

        let old_weight = self.heap[index].weight;
        let boxed_value = self.heap[index].value;
        self.heap[index] = Item{value: boxed_value, weight: new_weight};

        if new_weight < old_weight {
            self.percolate_up(index, new_weight);
        } else {
            self.percolate_down(index, new_weight);
        }
    }

    pub fn len(&self) -> usize {
        return self.heap.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    fn swap(&mut self, a: usize, b: usize) {
        let a_value = self.heap[a].value;
        let b_value = self.heap[b].value;

        self.heap.swap(a, b);
        self.indices.insert(a_value, b);
        self.indices.insert(b_value, a);
    }

    fn parent_index(&mut self, index: usize) -> usize {
        if index == 0 {
            return 0;
        }

        return (index - 1) / 2;
    }

    fn percolate_up(&mut self, index: usize, weight: f64) {
        let mut current_index = index;
        let mut parent_index = self.parent_index(current_index);

        while current_index > 0 && self.heap[parent_index].weight > weight {
            self.swap(current_index, parent_index);
            current_index = parent_index;
            parent_index = self.parent_index(current_index);
        }
    }

    fn percolate_down(&mut self, index: usize, weight: f64) {
        let mut has_completed = false;
        let mut current_index = index;

        while !has_completed {
            let left_index = 2 * current_index + 1;
            let right_index = 2 * current_index + 2;

            if 
                right_index < self.heap.len() && 
                self.heap[right_index].weight < weight &&
                self.heap[left_index].weight > self.heap[right_index].weight
            {
                self.swap(current_index, right_index);
                current_index = right_index;
            } else if 
                left_index < self.heap.len() && 
                self.heap[left_index].weight < weight
            {
                self.swap(current_index, left_index);
                current_index = left_index;
            } else {
                has_completed = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_heap<'a>() -> BinaryHeap<'a, i64> {
        let mut heap = BinaryHeap::<i64>::new();
        heap.push(&5, 5.0);
        heap.push(&1, 1.0);
        heap.push(&4, 4.0);
        heap.push(&2, 2.0);
        heap.push(&3, 3.0);
        return heap;
    }

    fn force_pop(heap: &mut BinaryHeap<i64>) -> i64 {
        return *heap.pop().expect("Heap was empty").value;
    }

    #[test]
    fn remove_all() {
        let mut heap = create_heap();
        assert_eq!(force_pop(&mut heap), 1);
        assert_eq!(force_pop(&mut heap), 2);
        assert_eq!(force_pop(&mut heap), 3);
        assert_eq!(force_pop(&mut heap), 4);
        assert_eq!(force_pop(&mut heap), 5);
    }

    #[test]
    fn is_empty() {
        let mut heap = BinaryHeap::<i64>::new();
        assert!(heap.is_empty());
        heap.push(&1, 1.0);
        assert!(!heap.is_empty());
        heap.pop();
        assert!(heap.is_empty());
    }

    #[test]
    fn length() {
        let mut heap = create_heap();
        assert_eq!(heap.len(), 5);
        heap.pop();
        assert_eq!(heap.len(), 4);
    }
    
    #[test]
    fn update_weight() {
        let mut heap = create_heap();
        heap.update_weight(&5, -3.0);
        assert_eq!(force_pop(&mut heap), 5);
    }
}