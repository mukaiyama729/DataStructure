use std::ops::Add;

pub struct Heap<Node>
where
    Node: Ord,
{
    pub heap: Vec<Node>,
}

impl<Node: Ord> Heap<Node> {
    fn push(&mut self, node: Node) -> () {
        self.heap.push(node);
        let mut i = self.heap.len();

        while i > 0usize {
            let p = (i - 1) / 2;
            if self.heap[p] <= self.heap[i] {
                break;
            }
            self.heap.swap(i, p);
            i = p;
        }
    }

    fn pop(&mut self) -> Option<Node> {
        let minimum_node = if self.heap.is_empty() {
            None
        } else {
            Some(self.heap.swap_remove(0))
        };
        let size = self.heap.len();

        let mut i = 0;

        while i * 2 + 1 < size {
            let mut target = i * 2 + 1;
            let left = &self.heap[target];
            let right = &self.heap[target + 1];
            if right < left {
                target = target + 1;
            }
            if &self.heap[target] <= &self.heap[i] {
                break;
            }
            self.heap.swap(target, i);
            i = target;
        }

        minimum_node
    }
}
