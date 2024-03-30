// スタックの実装

// NOTE.
// - capacity は未実装

pub struct Stack<T> {
    vec: Vec<T>,
}

pub trait Manipulation<T> {
    fn new() -> Self;
    fn push(&mut self, element: T);
    fn pop(&mut self) -> T;
    fn clear(&mut self);
    fn size(&self) -> u32;
    fn is_empty(&self) -> bool;
}

impl<T> Manipulation<T> for Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            vec: Vec::<T>::new(),
        }
    }

    fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    fn pop(&mut self) -> T {
        // 範囲外アクセスは panic を起こす
        self.vec.pop().unwrap()
    }

    fn clear(&mut self) {
        self.vec.clear();
    }

    fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }

    fn size(&self) -> u32 {
        self.vec.len() as u32
    }
}
