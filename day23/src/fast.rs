use crate::Computer;

#[derive(Debug)]
pub struct FastNetwork<const N: usize> {
    pub pcs: [Computer; N],
    pub map: [Selection<N>; N]
}

#[derive(Debug)]
pub struct Selection<const N: usize>(pub [bool; N]);