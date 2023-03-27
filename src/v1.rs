use std::collections::LinkedList;

pub struct FibonacciHeap<T> {
    roots: LinkedList<Tree<T>>,
}

impl<T: Ord> FibonacciHeap<T> {
    pub fn new() -> Self {
        Self {
            roots: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        count_nodes(&self.roots)
    }

    pub fn push(&mut self, item: T) {
        if self.peek().map(|o| &item <= o).unwrap_or(true) {
            // item is lt or eq to min value, or list is empty
            // push to front, becoming **new min**
            self.roots.push_front(Tree::new(item));
        } else {
            self.roots.push_back(Tree::new(item));
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.roots.front().map(Tree::root)
    }

    pub fn pop(&mut self) -> Option<T> {
        // take the front of the roots, since this is the _minimum_ value
        let Tree { node, children } = match self.roots.pop_front() {
            Some(x) => x,
            None => return None,
        };

        // add the child tree into the roots
        self.roots.extend(children);

        // perform the grouping of like-degrees
        rebalance(&mut self.roots);

        // find the minimum root value
        bring_min_to_front(&mut self.roots);

        Some(node)
    }
}

impl<T: Ord> FromIterator<T> for FibonacciHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);
        heap
    }
}

impl<T: Ord> Extend<T> for FibonacciHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter.into_iter() {
            self.push(x);
        }
    }
}

/// Rebalances the list of roots such that no two roots share the same degree.
/// The method employed uses a temporary array to order the trees by degrees.
/// This has a worst case of `O(n)` but is _amortised_ as `O(log n)`.
fn rebalance<T: Ord>(roots: &mut LinkedList<Tree<T>>) {
    if roots.is_empty() {
        return;
    }

    let nodes = count_nodes(roots);

    // NOTE: this will panic if nodes == 0
    let cap = nodes.ilog2() + 1;

    // initialise temp array with log2 of length
    let mut buf: Vec<Option<Tree<T>>> =
        std::iter::repeat_with(|| None).take(cap as usize).collect();

    // iterate through the roots
    while let Some(mut tree) = roots.pop_front() {
        loop {
            let degree = tree.degree();
            debug_assert!(
                degree < cap as usize,
                "degree is greater than log2(len) + 1"
            );

            // if a tree returns here, we need to repeat the loop since
            // the degrees would have increased by one
            tree = match buf[degree].take() {
                // most simple, slot was unoccupied so we just
                // insert tree into it and stop the loop
                None => {
                    buf[degree] = Some(tree);
                    break;
                }
                // there was already a tree with the same degree
                // and the new tree has a lesser root value
                // make the old tree a child of the new one
                Some(tree_b) if tree.root() <= tree_b.root() => {
                    tree.children.push(tree_b);
                    tree
                }
                // there was already a tree with the same degree
                // and the new tree has a greater root value
                // make the new tree a child of the old one
                Some(mut tree_b) => {
                    tree_b.children.push(tree);
                    tree_b
                }
            };
        }
    }

    // place the roots back into the linked list
    roots.extend(buf.into_iter().filter_map(|x| x));
}

fn count_nodes<T>(xs: &LinkedList<Tree<T>>) -> usize {
    xs.iter().map(Tree::count_nodes).sum()
}

fn bring_min_to_front<T: Ord>(roots: &mut LinkedList<Tree<T>>) {
    let min_index = roots
        .iter()
        .enumerate()
        .min_by_key(|(_, t)| t.root())
        .map(|(idx, _)| idx);

    if let Some(idx) = min_index {
        // split off returns a LinkedList with `idx` at the front (our minimum)
        let mut split = roots.split_off(idx);
        // append the remaining roots to the **end** of the split
        split.append(roots);
        // set the roots reference to be the new list
        *roots = split;
    }
}

struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> {
    fn new(root: T) -> Self {
        Self {
            node: root,
            children: Vec::new(),
        }
    }

    fn root(&self) -> &T {
        &self.node
    }

    fn degree(&self) -> usize {
        self.children.len()
    }

    fn count_nodes(&self) -> usize {
        1 + self.children.iter().map(Self::count_nodes).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::*;

    #[quickcheck]
    fn min_heap_property(xs: Vec<u32>) {
        let mut ll = LinkedList::from_iter(xs.into_iter().map(Tree::new));
        rebalance(&mut ll);

        // verify that all degrees are unique
        // we can leverage the fact that degrees are in _ascending_ order
        for (a, b) in ll.iter().zip(ll.iter().skip(1)) {
            assert!(a.degree() < b.degree(), "should have unique degrees");
        }

        for t in &ll {
            verify_min_heap(t);
        }

        // check that a rebalance does not break it
        rebalance(&mut ll);
    }

    #[quickcheck]
    fn recycle_on_min(xs: Vec<u32>) {
        let min = xs.iter().min().copied();
        let mut ll = LinkedList::from_iter(xs.into_iter().map(Tree::new));
        bring_min_to_front(&mut ll);

        assert_eq!(min.as_ref(), ll.front().map(Tree::root));

        rebalance(&mut ll);
        bring_min_to_front(&mut ll);
        assert_eq!(min.as_ref(), ll.front().map(Tree::root));
    }

    #[quickcheck]
    fn push_maintains_peek_min(xs: Vec<u32>) {
        let mut heap = FibonacciHeap::new();

        for (i, x) in xs.into_iter().enumerate() {
            if i % 4 == 0 {
                heap.pop();
            } else {
                let min = heap.peek().copied();
                heap.push(x);
                match min {
                    Some(x_) if x > x_ => assert_eq!(min.as_ref(), heap.peek()),
                    None | Some(_) => assert_eq!(Some(&x), heap.peek()),
                }
            }
        }
    }

    #[quickcheck]
    fn counting_nodes(xs: Vec<u32>) {
        let a = xs.len();

        let mut heap = FibonacciHeap::new();
        for x in xs {
            heap.push(x);
        }

        assert_eq!(heap.len(), a);

        heap.pop();
        assert_eq!(heap.len(), a.saturating_sub(1));
    }

    #[quickcheck]
    fn pops_by_min(xs: Vec<u32>) {
        pops_by_min_check(xs);
    }

    #[test]
    fn pops_by_min_01() {
        pops_by_min_check(vec![0, 0, 0, 1, 1]);
    }

    fn pops_by_min_check(mut xs: Vec<u32>) {
        let mut heap = FibonacciHeap::new();

        for x in &xs {
            heap.push(*x);
        }

        xs.sort();
        xs.reverse();

        while let Some(b) = heap.pop() {
            let a = xs.pop();
            assert_eq!(a, Some(b), "should in pop ascending order");
        }
    }

    fn verify_min_heap<T: Ord>(tree: &Tree<T>) {
        let Tree { node, children } = tree;
        for child in children {
            assert!(node <= child.root(), "node is lt or eq to child");
            verify_min_heap(child);
        }
    }
}
