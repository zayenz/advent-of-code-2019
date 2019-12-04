#[derive(Debug, Clone)]
pub struct UnionFind {
    nodes: Vec<i32>,
}

impl UnionFind {
    /// Create new UnionFind disjoint set that has nodes in [0..size)
    pub fn new(size: i32) -> UnionFind {
        let mut nodes = Vec::with_capacity(size as usize);
        for _ in 1..=size {
            nodes.push(-1);
        }
        UnionFind { nodes }
    }

    /// The root node of node
    pub fn find(&self, node: i32) -> i32 {
        assert!(0 <= node && (node as usize) < self.nodes.len());
        let root = self.nodes[node as usize];
        if root >= 0 {
            self.find(root)
        } else {
            node
        }
    }

    /// The root node of node, copacting the path along the way
    fn find_compacting(&mut self, node: i32) -> i32 {
        assert!(0 <= node && (node as usize) < self.nodes.len());
        let parent = self.nodes[node as usize];
        if parent >= 0 {
            let root = self.find(parent);
            if root != parent {
                self.nodes[node as usize] = root;
            }
            root
        } else {
            node
        }
    }

    /// Join the two groups represented by node1 and node2
    pub fn join(&mut self, node1: i32, node2: i32) {
        let root1 = self.find_compacting(node1);
        let root2 = self.find_compacting(node2);

        if root1 != root2 {
            let (larger_root, smaller_root) = if root1 < root2 {
                (root1, root2)
            } else {
                (root2, root1)
            };

            self.nodes[larger_root as usize] += self.nodes[smaller_root as usize];
            self.nodes[smaller_root as usize] = larger_root;
        }
    }

    /// The size of the group node belongs to
    pub fn group_size(&self, node: i32) -> i32 {
        let root = self.find(node);
        self.nodes[root as usize].abs()
    }

    /// The numebr of groups
    pub fn group_count(&self) -> i32 {
        let mut result = 0;
        for &node in &self.nodes {
            if node < 0 {
                result += 1;
            }
        }
        result
    }
}
