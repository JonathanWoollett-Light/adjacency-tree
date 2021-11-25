use std::ops::{Index, IndexMut};

pub const fn ac(x: u32) -> usize {
    (3u32.pow(x) - 1u32) as usize
}
/// The field of nodes adjacent to a node in a tree of a given dimensionality.
///
/// With 1, you have:
/// <table>
///    <tr><td>left</td><td>center</td><td>right</td></tr>
/// </table>
/// With 2, you have:
/// <table>
///    <tr><td>up left</td><td>up</td><td>up right</td></tr>
///    <tr><td>left</td><td>center</td><td>right</td></tr>
///    <tr><td>down left</td><td>down</td><td>down right</td></tr>
/// </table>
/// Etc.
// TODO Double check deriving this `Clone` actually works? (does it not require clone be implemented on `T`?)
#[derive(Debug,Clone)]
pub struct Adjacents<T, const D: u32>([T; ac(D)])
where
    [(); ac(D)]: ;

impl<T, const D: u32> Index<usize> for Adjacents<T, D>
where
    [(); ac(D)]: ,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, const D: u32> IndexMut<usize> for Adjacents<T, D>
where
    [(); ac(D)]: ,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
impl<T: Default> Default for Adjacents<T, 1u32> {
    fn default() -> Self {
        Self([T::default(), T::default()])
    }
}
impl<T: Default> Default for Adjacents<T, 2u32> {
    fn default() -> Self {
        Self([
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
        ])
    }
}

pub type A1 = Adjacents<(), 1>;
pub type A2 = Adjacents<(), 2>;

#[allow(unused)]
impl<T> Adjacents<T, 1u32> {
    pub const LEFT: usize = 0;
    pub const CENTER: usize = 1;
    pub const RIGHT: usize = 2;
    // TODO Make this keep working, if `UP_LEFT`, `UP_RIGHT` etc. change.
    pub fn new(left: T, right: T) -> Self {
        Self([left, right])
    }
}
#[allow(unused)]
impl<T> Adjacents<T, 2u32> {
    pub const UP_LEFT: usize = 0;
    pub const UP: usize = 1;
    pub const UP_RIGHT: usize = 2;
    pub const LEFT: usize = 3;
    pub const RIGHT: usize = 4;
    pub const DOWN_LEFT: usize = 5;
    pub const DOWN: usize = 6;
    pub const DOWN_RIGHT: usize = 7;
    // TODO Make this keep working, if `UP_LEFT`, `UP_RIGHT` etc. change.
    pub fn new(
        up_left: T,
        up: T,
        up_right: T,
        left: T,
        right: T,
        down_left: T,
        down: T,
        down_right: T,
    ) -> Self {
        Self([
            up_left, up, up_right, left, right, down_left, down, down_right,
        ])
    }
}
