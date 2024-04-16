use core::ops::{Add, Div, Mul, Sub};
use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

pub struct Complex<T>
where T: Copy,
{
    pub re: T,
    pub im: T
}

impl<T> Complex<T>
    where T: Copy,
{
    pub fn new(re: T, im: T) -> Self{
        Complex{
            re,
            im
        }
    }
}

impl<T> Add for Complex<T>
    where T: Copy + Add<T, Output = T>{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> Sub for Complex<T>
    where T: Copy + Sub<T, Output = T>{
    type Output = Complex<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl<T> Mul for Complex<T>
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T>{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex{
            re: (self.re * rhs.re) - (self.im * rhs.im),
            im: (self.re * rhs.im) + (rhs.re * self.im)
        }
    }
}

impl<T> Div for Complex<T>
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> +  Div<T, Output = T>, {
    type Output = Complex<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Complex{
            re: ((self.re * rhs.re) + (self.im * rhs.im)) / ((rhs.re * rhs.re) + (rhs.im * rhs.im)),
            im: ((rhs.re * self.im) - (self.re * rhs.im)) / ((rhs.re * rhs.re) + (rhs.im * rhs.im))
        }
    }
}

impl<T> Clone for Complex<T> where T: Copy {
    fn clone(&self) -> Self {
        Complex{
            re: self.re,
            im: self.im
        }
    }
}

impl<T> Copy for Complex<T>
    where T: Copy {
    
}