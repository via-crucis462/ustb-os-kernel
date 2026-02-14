type StrideInner = usize;
#[derive(Default, Clone, Copy)]
pub struct Stride(StrideInner);

impl Stride {
    const BIG_STRIDE: StrideInner = StrideInner::MAX / 10000;

    pub fn step(&mut self, prio: usize) {
        let pass = Stride::BIG_STRIDE / prio;
        self.0 += pass;
    }
}