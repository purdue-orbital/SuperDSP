#![no_std]

mod stages;

extern crate alloc;

extern crate core;


pub trait Stage<Args> {
    fn invoke(&mut self, args: Args);
}

impl<F> Stage<()> for F where
    F: FnMut(),
{
    fn invoke(&mut self, args: ()) {
        self()
    }
}

impl<F, A> Stage<(A,)> for F where
    F: FnMut(A),
{
    fn invoke(&mut self, args: (A,)) {
        self(args.0)
    }

}

impl<F, A, B> Stage<(A, B)> for F where
    F: FnMut(A, B),
{
    fn invoke(&mut self, args: (A, B)) {
        self(args.0, args.1)
    }
}

impl<F, A, B, C> Stage<(A, B, C)> for F where
    F: FnMut(A, B, C),
{
    fn invoke(&mut self, args: (A, B, C)) {
        self(args.0, args.1, args.2)
    }
}

impl<F, A, B, C, D> Stage<(A, B, C, D)> for F where
    F: FnMut(A, B, C, D),
{
    fn invoke(&mut self, args: (A, B, C, D)) {
        self(args.0, args.1, args.2, args.3)
    }
}

impl<F, A, B, C, D, E> Stage<(A, B, C, D, E)> for F where
    F: FnMut(A, B, C, D, E),
{
    fn invoke(&mut self, args: (A, B, C, D, E)) {
        self(args.0, args.1, args.2, args.3, args.4)
    }
}

impl<F, A, B, C, D, E, G> Stage<(A, B, C, D, E, G)> for F where
    F: FnMut(A, B, C, D, E, G),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5)
    }
}

impl<F, A, B, C, D, E, G, H> Stage<(A, B, C, D, E, G, H)> for F where
    F: FnMut(A, B, C, D, E, G, H),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6)
    }
}

impl<F, A, B, C, D, E, G, H, I> Stage<(A, B, C, D, E, G, H, I)> for F where
    F: FnMut(A, B, C, D, E, G, H, I),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7)
    }
}

impl<F, A, B, C, D, E, G, H, I, J> Stage<(A, B, C, D, E, G, H, I, J)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K> Stage<(A, B, C, D, E, G, H, I, J, K)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L> Stage<(A, B, C, D, E, G, H, I, J, K, L)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L, M> Stage<(A, B, C, D, E, G, H, I, J, K, L, M)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L, M),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L, M)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10, args.11)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L, M, N> Stage<(A, B, C, D, E, G, H, I, J, K, L, M, N)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L, M, N),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L, M, N)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10, args.11, args.12)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L, M, N, O> Stage<(A, B, C, D, E, G, H, I, J, K, L, M, N, O)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L, M, N, O),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L, M, N, O)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10, args.11, args.12, args.13)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L, M, N, O, P> Stage<(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L, M, N, O, P)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10, args.11, args.12, args.13, args.14)
    }
}

impl<F, A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, Q> Stage<(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, Q)> for F where
    F: FnMut(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, Q),
{
    fn invoke(&mut self, args: (A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, Q)) {
        self(args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10, args.11, args.12, args.13, args.14, args.15)
    }
}

pub type Frequency = f32;
pub type CarrierFrequency = Frequency;
pub type Gain = u32;
pub type SampleRate = f32;
pub type SamplesPerSymbol = usize;
pub type NumberOfTaps = usize;

