/* 二分木 */
#[derive(Debug)]
pub enum BinTree<T> {
    Nil,
    Node{
        val: T,
        left: Box<BinTree<T>>,
        right: Box<BinTree<T>>,
    },
}
impl<T> BinTree<T> {
    pub fn postorder(&self, act: &dyn Fn(&T))  {
        BinTree::<T>::postorder_in(self, act)
    }

    fn postorder_in(t: &BinTree<T>, act: &dyn Fn(&T)){
        match t {
            BinTree::<T>::Nil => (),
            BinTree::<T>::Node{val, left, right} => {
                BinTree::<T>::postorder_in(&*left, act);
                BinTree::<T>::postorder_in(&*right, act);
                act(val)
            }
        }
    }

    pub fn new_leaf(k: T) -> BinTree<T> {
        BinTree::new_node(k, BinTree::<T>::Nil, BinTree::<T>::Nil)
    }

    pub fn new_node(k: T, l: BinTree::<T>, r: BinTree::<T>) -> BinTree<T> {
        BinTree::<T>::Node {
            val: k,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
}