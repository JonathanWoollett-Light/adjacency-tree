use std::ops::{Index, IndexMut};

pub const fn ac(x: u32) -> usize {
    2u32.pow(x) as usize
}
/// The children of a node in a tree of a given dimensionality.
///
/// With 1, you have:
/// <table>
///    <tr><td>left</td><td>right</td></tr>
/// </table>
/// With 2, you have:
/// <table>
///    <tr><td>up left</td><td>up right</td></tr>
///    <tr><td>down left</td><td>down right</td></tr>
/// </table>
/// Etc.
#[derive(Debug)]
pub struct Children<T, const D: u32>(pub [T; ac(D)])
where
    [(); ac(D)]: ;

impl<T, const D: u32> Index<usize> for Children<T, D>
where
    [(); ac(D)]: ,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, const D: u32> IndexMut<usize> for Children<T, D>
where
    [(); ac(D)]: ,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

pub type C1 = Children<(), 1>;
pub type C2 = Children<(), 2>;

#[allow(unused)]
impl<T> Children<T, 1u32> {
    pub const LEFT: usize = 0;
    pub const RIGHT: usize = 1;
    // TODO Make this keep working, if `UP_LEFT`, `UP_RIGHT` etc. change.
    pub fn new(left: T, right: T) -> Self {
        Self([left, right])
    }
}

#[allow(unused)]
impl<T> Children<T, 2u32> {
    pub const UP_LEFT: usize = 0;
    pub const UP_RIGHT: usize = 1;
    pub const DOWN_LEFT: usize = 2;
    pub const DOWN_RIGHT: usize = 3;
    // TODO Make this keep working, if `UP_LEFT`, `UP_RIGHT` etc. change.
    pub fn new(up_left: T, up_right: T, down_left: T, down_right: T) -> Self {
        Self([up_left, up_right, down_left, down_right])
    }
}
impl<T> Into<(T, T)> for Children<T, 1u32> {
    // TODO This is really bad, do this better.
    fn into(self) -> (T, T) {
        let mut first = Vec::from(self.0);
        let mut second = first.split_off(1);
        (first.pop().unwrap(), second.pop().unwrap())
    }
}
