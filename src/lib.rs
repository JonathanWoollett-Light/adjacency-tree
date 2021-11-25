#![feature(generic_const_exprs)]
// I don't care `generic_const_exprs` is incomplete.
#![allow(incomplete_features)]
// Basically everything throws the `const_evaluatable_unchecked` warning in this library.
#![allow(const_evaluatable_unchecked)]


// TODO Remove this (just here for development rn).
#![allow(dead_code)]

//! Many implementations could be brought together and made generic over dimensionality, but I
//!  personally find the additional steps of abstraction required to do this, to add unwarranted
//!  difficulty in understanding how this works and thus I would argue in favour of the code
//!  duplication for the sake of explicitness.

pub mod adjacents;
pub mod children;

#[cfg(test)]
mod test;

use adjacents::{Adjacents, A1};
use children::{Children,C1};
use std::{fmt::Debug, sync::{Arc, RwLock}, u32};

pub enum SplitRes<P, T, const D: u32>
where
    [(); children::ac(D)]: ,
{
    Split(Children<P, D>),
    Into(T),
}
pub trait Split<T, const D: u32>: Sized
where
    [(); children::ac(D)]: ,
{
    fn split(&self) -> SplitRes<Self, T, D>;
}

// TODO Can this reasonably not be pub?
// Please ignore this.
pub const fn root_ac(x: u32) -> usize {
    3u32.pow(x) as usize
}
/// The adjacency tree.
pub struct AdjacencyTree<T, const D: u32>
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ,
    [(); root_ac(D)]: 
{
    root_children: [Child<T, D>; root_ac(D)],
}
impl<T: Debug> AdjacencyTree<T, 1u32> {
    /// Given `x` and the implementation of `Split` on `P`, splits `x` to form an adjacency tree. 
    pub fn split_from<P: Split<T, 1u32>>(x: P) -> Self {
        let (root, depth) = Child::split_from(x, 0);
        Self {
            root_children: [
                Child::border_left(depth),
                root,
                Child::border_right(depth),
            ],
        }
    }
    fn set_adjacents(&mut self) {
        self.root_children[1].set_adjacents(
            Adjacents::<_,1u32>::new(self.root_children[0].clone(),self.root_children[2].clone())
        );
    }
}

/// A child node.
#[derive(Debug)]
pub enum Child<T, const D: u32>
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ,
{
    Leaf(Leaf<T, D>),
    Branch(Branch<T, D>),
}
impl<T:Debug, const D:u32> Child<T,D> where
[(); adjacents::ac(D)]: ,
[(); children::ac(D)]: ,{
    fn leaf(&self) -> &Leaf<T,D> {
        match self {
            Self::Leaf(l) => &l,
            Self::Branch(_) => panic!("called `Child::Leaf()` on a `Branch` value")
        }
    }
    fn branch(&self) -> &Branch<T,D> {
        match self {
            Self::Branch(b) => &b,
            Self::Leaf(_) => panic!("called `Child::Branch()` on a `Leaf` value"),
        }
    }
}
impl<T:Debug, const D:u32> Clone for Child<T,D> where
[(); adjacents::ac(D)]: ,
[(); children::ac(D)]: ,{
    fn clone(&self) -> Self {
        match self {
            Self::Leaf(l) => Self::Leaf(l.clone()),
            Self::Branch(b) => Self::Branch(b.clone())
        }
    }
}
impl<T: Debug> Child<T, 1u32> {
    fn border_right(depth: usize) -> Self {
        match depth {
            0 => Self::Leaf(Default::default()),
            _ => Self::Branch(Branch::border_right(depth)),
        }
    }
    fn border_left(depth: usize) -> Self {
        match depth {
            0 => Self::Leaf(Default::default()),
            _ => Self::Branch(Branch::border_left(depth)),
        }
    }
    fn set_adjacents(&mut self, adjacents: Adjacents<Self,1u32>) {
        match self {
            Self::Leaf(l) => {
                l.set_adjacents(Adjacents::<_,1u32>::new(
                    adjacents[A1::LEFT].leaf().clone(),
                    adjacents[A1::RIGHT].leaf().clone()
                ));
            }
            Self::Branch(b) => {
                b.set_adjacents(Adjacents::<_,1u32>::new(
                    adjacents[A1::LEFT].branch().clone(),
                    adjacents[A1::RIGHT].branch().clone()
                ));
            }
        }
    }
}
impl<T: Debug> Child<T, 1u32> {
    fn split_from<P: Split<T, 1u32>>(x: P, depth: usize) -> (Self, usize) {
        match x.split() {
            SplitRes::Split(split) => {
                let (branch, depth) = Branch::split_from(split, depth);
                (Self::Branch(branch), depth)
            }
            SplitRes::Into(into) => (Self::Leaf(Leaf::<_, 1u32>::new(into)), depth),
        }
    }
}
/// The alias for wrapper types around all child nodes.
type NodeWrapper<T> = Arc<RwLock<Option<T>>>;
#[derive(Debug)]
/// A branch node.
pub struct Branch<T, const D: u32>(NodeWrapper<BranchData<T, D>>)
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ;
impl<T: Debug> Branch<T, 1u32> {
    fn border_left(depth: usize) -> Self {
        Self(Arc::new(RwLock::new(Some(BranchData::border_left(depth)))))
    }
    fn border_right(depth: usize) -> Self {
        Self(Arc::new(RwLock::new(Some(BranchData::border_right(depth)))))
    }
    fn set_adjacents(&mut self, adjacents: Adjacents<Self,1u32>) {
        if let Some(branch_data) = (*self.0.write().unwrap()).as_mut() {
            branch_data.set_adjacents(adjacents);
        }
    }
}
impl<T: Debug> Branch<T, 1u32> {
    fn split_from<P: Split<T, 1u32>>(children: Children<P, 1u32>, depth: usize) -> (Self, usize) {
        let (branch, depth) = BranchData::split_from(children, depth);
        (Self(Arc::new(RwLock::new(Some(branch)))), depth)
    }
}
// TODO Why doesn't this work by deriving it?
impl<T: Debug, const D: u32> Clone for Branch<T, D>
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
// TODO Why doesn't this work by deriving it?
impl<T: Debug, const D: u32> Default for Branch<T, D>
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ,
{
    fn default() -> Self {
        // Arc::new(RwLock::new(None))
        Self(Default::default())
    }
}
#[derive(Debug)]
/// A branch node's data.
pub struct BranchData<T, const D: u32>
where
    [(); adjacents::ac(D)]: ,
    [(); children::ac(D)]: ,
{
    adjacent: Adjacents<Branch<T, D>, D>,
    children: Children<Child<T, D>, D>,
}
impl<T: Debug> BranchData<T, 1u32> {
    fn split_from<P: Split<T, 1u32>>(children: Children<P, 1u32>, depth: usize) -> (Self, usize) {
        let (t0, t1) = children.into();
        let ((c0, d0), (c1, d1)) = (
            Child::split_from(t0, depth + 1),
            Child::split_from(t1, depth + 1),
        );
        (
            Self {
                adjacent: Default::default(),
                children: Children([c0, c1]),
            },
            std::cmp::max(d0, d1),
        )
    }
    fn set_adjacents(&mut self, adjacents: Adjacents<Branch<T, 1u32>,1u32>) {
        self.adjacent = adjacents.clone();
        // --------------------------------------------------------------------------
        // --------------------------------------------------------------------------
        // ---------------START BACK HERE WHEN YOU GOT MORE MOTIVATION---------------
        // --------------------------------------------------------------------------
        // THIS IS BG AREA WHERE WORKS NEEDS TO BE DONE TO ACCOUNT FOR INTERNAL VOIDS
        // --------------------------------------------------------------------------
        // --------------------------------------------------------------------------

        // Since we have the border nodes, we can call `unwrap()` on 
        //  `self.adjacent[_].0.read().unwrap()` since we know this section will have an adjacent 
        //  section.
        
        // Sets adjacent for left child.
        let right = self.children[C1::RIGHT].clone();
        self.children[C1::LEFT].set_adjacents(Adjacents::<_,1u32>::new(
            (*self.adjacent[A1::LEFT].0.read().unwrap()).as_ref().unwrap().children[C1::RIGHT].clone(),
            right
        ));
        // Sets adjacent for right child.
        let left = self.children[C1::LEFT].clone();
        self.children[C1::RIGHT].set_adjacents(Adjacents::<_,1u32>::new(
            left,
            (*self.adjacent[A1::RIGHT].0.read().unwrap()).as_ref().unwrap().children[C1::LEFT].clone(),
        ));
    }
}
impl<T: Debug> BranchData<T, 1u32> {
    fn border_left(depth: usize) -> Self {
        let children = match depth {
            0 => unreachable!(),
            1 => Children([
                Child::Leaf(Default::default()),
                Child::Leaf(Default::default()),
            ]),
            _ => Children([
                Child::Branch(Default::default()),
                Child::Branch(Branch::border_left(depth - 1)),
            ]),
        };
        Self {
            adjacent: Default::default(),
            children,
        }
    }
    fn border_right(depth: usize) -> Self {
        let children = match depth {
            0 => unreachable!(),
            1 => Children([
                Child::Leaf(Default::default()),
                Child::Leaf(Default::default()),
            ]),
            _ => Children([
                Child::Branch(Branch::border_right(depth - 1)),
                Child::Branch(Default::default()),
            ]),
        };
        Self {
            adjacent: Default::default(),
            children,
        }
    }
}
#[derive(Debug)]
/// A leaf node.
pub struct Leaf<T, const D: u32>(NodeWrapper<LeafData<T, D>>)
where
    [(); adjacents::ac(D)]: ;
impl<T> Leaf<T, 1u32> {
    fn new(data: T) -> Self {
        Self(Arc::new(RwLock::new(Some(LeafData::<_, 1u32>::new(data)))))
    }
    fn set_adjacents(&mut self, adjacents: Adjacents<Self,1u32>) {
        self.
    }
}
impl<T> Leaf<T, 2u32> {
    fn new(data: T) -> Self {
        Self(Arc::new(RwLock::new(Some(LeafData::<_, 2u32>::new(data)))))
    }
}
// TODO Why doesn't this work by deriving it?
impl<T: Debug, const D: u32> Clone for Leaf<T, D>
where
    [(); adjacents::ac(D)]: ,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
// TODO Why doesn't this work by deriving it?
impl<T, const D: u32> Default for Leaf<T, D>
where
    [(); adjacents::ac(D)]: ,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
#[derive(Debug)]
/// A leaf node's data.
pub struct LeafData<T, const D: u32>
where
    [(); adjacents::ac(D)]: ,
{
    adjacent: Adjacents<Leaf<T, D>, D>,
    data: T,
}
impl<T> LeafData<T, 1u32> {
    fn new(data: T) -> Self {
        Self {
            adjacent: Default::default(),
            data,
        }
    }
    fn set_adjacents(&mut self, adjacents: Adjacents<Leaf<T, 1u32>,1u32>) {
        self.adjacent = adjacents;
    }
}
impl<T> LeafData<T, 2u32> {
    fn new(data: T) -> Self {
        Self {
            adjacent: Default::default(),
            data,
        }
    }
}
