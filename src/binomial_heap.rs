use std::collections::LinkedList;

use crate::binary_heap::Item;

type NodeList<T> = LinkedList<Node<T>>;

#[derive(Debug)]
struct Node<T> {
    height: usize,
    value: T,
    weight: f64,
    children: NodeList<T>,
}

fn meld_nodes<T>(mut left: Node<T>, mut right: Node<T>) -> Node<T> {
    assert_eq!(left.height, right.height);

    if left.weight < right.weight {
        left.children.push_back(right);
        return Node {
            height: left.height + 1,
            weight: left.weight,
            value: left.value,
            children: left.children,
        }
    }

    right.children.push_back(left);
    return Node { 
        height: right.height + 1, 
        value: right.value, 
        weight: right.weight, 
        children: right.children,
    }
}

#[derive(Debug)]
pub struct BinomialHeap<T> {
    trees: NodeList<T>,
    len: usize,
}

fn meld_trees<'a, T>(our_trees: &mut NodeList<T>, their_trees: &mut NodeList<T>) -> NodeList<T> {
    let mut new_trees: NodeList<T> = LinkedList::new();
    let mut carry: Option<Node<T>> = None;

    while !our_trees.is_empty() || !their_trees.is_empty() {
        if let None = carry {
            if our_trees.is_empty() {
                new_trees.append(their_trees);
                return new_trees;
            } else if their_trees.is_empty() {
                new_trees.append(our_trees);
                return new_trees
            }
        }

        let (current, new_carry) = match (our_trees.front(), their_trees.front()) {
            (None, Some(_)) => (their_trees.pop_front(), None),
            (Some(_), None) => (our_trees.pop_front(), None),
            (Some(our_node), Some(their_node)) =>
                if our_node.height < their_node.height {
                    (our_trees.pop_front(), None)
                } else if their_node.height < our_node.height {
                    (their_trees.pop_front(), None)
                } else {
                    let new_node = meld_nodes(
                        our_trees.pop_front().expect("List won't be empty"), 
                        their_trees.pop_front().expect("List won't be empty")
                    );
                    (None, Some(new_node))
                }
            (None, None) => panic!("Both trees should be nonempty")
        };

        match (current, carry) {
            (None, Some(carry_node)) => {
                new_trees.push_back(carry_node);
                carry = new_carry;
            },
            (Some(current_node), Some(carry_node)) => {
                if carry_node.height < current_node.height {
                    new_trees.push_back(carry_node);
                    new_trees.push_back(current_node);
                    carry = new_carry;
                } else {
                    carry = Some(
                        meld_nodes(current_node, carry_node)
                    )
                }
            },
            (current, None) => {
                if let Some(current_node) = current {
                    new_trees.push_back(current_node);
                }

                carry = new_carry;
            }
        }
    }

    if let Some(carry_node) = carry {
        new_trees.push_back(carry_node);
    }

    return new_trees;
}


impl<T> BinomialHeap<T> {
    pub fn new() -> BinomialHeap<T> {
        return BinomialHeap {
            trees: LinkedList::new(),
            len: 0,
        }
    }

    pub fn meld(&mut self, mut other: BinomialHeap<T>) {
        self.trees = meld_trees(&mut self.trees, &mut other.trees);
        self.len += other.len;
    }

    pub fn push(&mut self, value: T, weight: f64) {
        let node = Node {
            height: 0,
            value: value,
            weight,
            children: LinkedList::new(),
        };
        let mut tree: NodeList<T>= LinkedList::new();
        tree.push_back(node);
        self.trees = meld_trees(&mut self.trees, &mut tree);
        self.len += 1;
    } 

    pub fn pop(&mut self) -> Option<Item<T>> {
        if self.len == 0 {
            return None;
        } 

        let (_, min_index, ) = self.trees.iter_mut().enumerate().fold((None, 0), |acc, (current_index, node)| {
            return match acc {
                (None, _) => (
                    Some(node), 
                    current_index, 
                ),
                (Some(min_node), min_index) =>
                    if min_node.weight <= node.weight {
                        (Some(min_node), min_index)
                    } else {
                        (Some(node), current_index)
                    }
            }
        });

        let mut second_half = self.trees.split_off(min_index);
        let mut min_node = second_half.pop_front().expect("Second half should contain something");
        self.trees.append(&mut second_half);
        self.trees = meld_trees(&mut self.trees, &mut min_node.children);

        let min_item = Item {
            value: min_node.value,
            weight: min_node.weight,
        };
        self.len -= 1;

        return Some(min_item);
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_heap<'a>(size: usize) -> BinomialHeap<usize> {
        let mut heap = BinomialHeap::<usize>::new();

        for i in 0..size {
            heap.push(i, i as f64);
        }
        return heap;
    }

    fn force_pop<T : Copy>(heap: &mut BinomialHeap<T>) -> T {
        return heap.pop().expect("Heap was empty").value;
    }

    fn contains_sequential(heap: &mut BinomialHeap<usize>, amount: usize) {
        for i in 0..amount {
            assert_eq!(force_pop(heap), i);
        }
    }

    #[test]
    fn remove_all() {
        let mut heap = create_heap(5);
        contains_sequential(&mut heap, 5)
    }

    #[test]
    fn is_empty() {
        let mut heap = BinomialHeap::<i64>::new();
        assert!(heap.is_empty());
        heap.push(1, 1.0);
        assert!(!heap.is_empty());
        heap.pop();
        assert!(heap.is_empty());
    }

    #[test]
    fn length() {
        let mut heap = create_heap(5);
        assert_eq!(heap.len(), 5);
        heap.pop();
        assert_eq!(heap.len(), 4);
    }
    
    #[test]
    fn merge_into_bigger() {
        let first_heap = create_heap(5);
        let mut second_heap = create_heap(0);
        for i in 5..15 {
            second_heap.push(i, i as f64);
        }
        second_heap.meld(first_heap);
        contains_sequential(&mut second_heap, 15);
    }
    
    #[test]
    fn merge_into_smaller() {
        let mut first_heap = create_heap(5);
        let mut second_heap = create_heap(0);
        for i in 5..15 {
            second_heap.push(i, i as f64);
        }
        first_heap.meld(second_heap);
        contains_sequential(&mut first_heap, 15);
    }
    
    #[test]
    fn insert_decreasing() {
        let mut heap = create_heap(0);
        for i in 0..100 {
            heap.push(99 - i, (99-i) as f64);
        }

        contains_sequential(&mut heap, 100);
    }

}