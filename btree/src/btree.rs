#![allow(dead_code)]
#![allow(unused_variables)]

type ChildNode<T> = Option<Box<BTNode<T>>>;

// our BTree will either be None(Empty) or Some BTNode
struct BinaryTree<T> {
    head: Option<BTNode<T>>
}

// since structs are not able to contain refs to itself we need to Box our 'pointers' in order to
// allow for this. exmaple of a recursive type which is very illegal in rust
struct BTNode<T> {
    // Using Option to handle node terminals aka end nodes
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>
}

enum Op<T> {
    Add,
    Sub,
    Div,
    Mul,
    Id(T)
}

impl BTNode<i32> {
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {
            op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r))
        }
    }
}

impl BinaryTree<i32> {
    pub fn new(head: BTNode<i32>) -> Self {
        BinaryTree::<i32> { head: Some(head) }
    }

    pub fn collapse(node: &Box<BTNode<i32>>) -> i32 {
        let mut r: Option<i32> = None;
        let mut l: Option<i32> = None;

        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }

        let r = if let Some(x) = r {x} else {0};
        let l = if let Some(x) = l {x} else {0};

        match node.op {
            Op::Add   => { l + r }
            Op::Sub   => { l - r }
            Op::Mul   => { l * r }
            Op::Id(x) => { x }
            Op::Div   => { 
                if r == 0 {
                    panic!("[Error] Attempted to divide by 0!");
                }
                l / r
            }
        }
    }
}

fn add_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Add, l, r)
}

fn sub_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Sub, l, r)
}

fn div_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Div, l, r)
}

fn mul_node(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Mul, l, r)
}

fn id_node(value: i32) -> BTNode<i32> {
    // not using the new constructor here, we construct a node directory since we don't really care
    // about the left and right 'pointers'
    BTNode {
        op: Op::Id(value),
        left: None,
        right: None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn operations() {
        let tree = BinaryTree::new(
            add_node(
                sub_node(
                    id_node(10),
                    mul_node(
                        id_node(2),
                        id_node(2)
                    )
                ),
                add_node(
                    id_node(8),
                    div_node(
                        id_node(10),
                        id_node(2)
                    )
                )
            )
        ); 

        let val = BinaryTree::collapse(&Box::new(tree.head.expect("No head init.")));
        println!("Value: {}", &val);

        assert_eq!(19, val);
    }
}
