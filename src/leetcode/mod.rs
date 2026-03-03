use std::cell::RefCell;
use std::rc::Rc;

mod p1;
mod p104;
mod p121;
mod p122;
mod p125;
mod p1351;
mod p1356;
mod p167;
mod p169;
mod p189;
mod p202;
mod p209;
mod p219;
mod p238;
mod p26;
mod p27;
mod p28;
mod p283;
mod p3;
mod p344;
mod p349;
mod p35;
mod p350;
mod p392;
mod p5;
mod p55;
mod p58;
mod p704;
mod p744;
mod p80;
mod p88;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
