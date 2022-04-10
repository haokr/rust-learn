use std::rc::Rc;

/// DAG
/// RC、RefCell 线程不安全，不可用于多线程
/// 多线程使用 Arc，如果要在多线程中使用内部可变性，使用 Mutex 和 RwLock

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
    // 内部使用 RefCell 再包一层，可以拿到内部数据的可变引用，来修改数据
    // downstream: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(node4));
    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let node5 = Node::new(5);
    // RC 是一个只读的引用计数，无法拿到内部数据的可变引用
    // 因此，如果想拿到内部数据的可变引用需要用 RefCell
    let node3 = node1.get_downstream().unwrap();
    node3.update_downstream(Rc::new(node5));
    println!("node1: {:?}, node2: {:?}", node1, node2);
}