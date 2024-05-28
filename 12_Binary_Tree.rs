/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Implementing `new`, `insert`, `len`, and `has` for `Subtree`.

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        // Create an empty Subtree
        Subtree(None)
    }
    
    fn insert(&mut self, value: T) {
    
        // To prevent any move semantics and be able to modify the value we need &mut
        match &mut self.0 {
        
            // Base case: if there is space here (nothing exists, hence none), why look anywhere else ;)
            None => self.0 = Some(Box::new(Node {
                                      value: value,
                                      left: Subtree(None),
                                      right: Subtree(None)
                                    })),
            
            // Other cases depend on whether the value is smaller or larger than the pre-existing value 
            Some(x) => {
                if value < x.value {
                    x.left.insert(value);
                    
                } else if value > x.value {
                   x.right.insert(value); 
                   
                } else {
                    // Ignore same values as we want unique ones
                    ()
                }
            },
            
        }
    }
    
    fn has(&self, value: &T) -> bool {
        
        let result: bool = match &self.0 {
            
            // Base case 1: Empty Subtree, return false 
            None => false,
            
            Some(x) => {
            
                // Base case 2: If found return true
                if *value == x.value {
                    true
                    
                } else {
                    // Other case: continue further down the tree, left Subtree and then the right one
                    // If either of the Subtrees contains it, then we have found it
                    x.left.has(value) || x.right.has(value)
                }
            }
        };
        
        return result;
    }
    
    fn len(&self) -> usize {
    
         let result: usize = match &self.0 {
             
             // Base case: If no element exists, then 0
             None => 0,
             
             // Otherwise it is current element (1) plus the ones in left and right Subtrees
             Some(x) => 1 + x.left.len() + x.right.len()
         };
         
         return result;
    }
}

// TODO: Bonus exercise of implementing an iterator over this binary tree

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
