use crate::math::matrix::Matrix;

pub trait Element<T,Y>{
    async fn run(&mut self, input: Matrix<T>) -> Matrix<Y>;
    async fn prepare(&mut self, input: Matrix<T>) -> Matrix<Y>;
}