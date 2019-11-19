pub trait Dimension {
    const INDEX: usize;
}

pub struct X;
pub struct Y;
pub struct Z;

impl Dimension for X {
    const INDEX: usize = 0;
}

impl Dimension for Y {
    const INDEX: usize = 1;
}

impl Dimension for Z {
    const INDEX: usize = 2;
}
