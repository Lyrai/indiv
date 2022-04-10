#[macro_export]
macro_rules! deref {
    ($node: expr) => { $node.borrow().deref() };
}