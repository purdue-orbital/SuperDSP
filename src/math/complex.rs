use core::ops::{Add, Div, Mul, Sub};
use core::f32::consts::{FRAC_2_PI, FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};
use std::f32::consts::FRAC_1_PI;

pub struct Complex<T>
where T: Copy,
{
    re: T,
    im: T
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


impl<T> Complex<T>
    where T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Into<f32> + From<f32>{

    fn cos_appox_2nd_order(val: f32) -> f32{
        ( -0.4848033041659485362470884342167246994959672938035335457857513967256210919426258097884883724562191232739132885763651631919209030859833904824813844878136432485909455005875182415401398818340676118871723072552574344546498607458911841330041 * (val * val)) + 1.0
    }
    
    fn abs(x: f32) -> f32{
        f32::from_bits(x.to_bits() & (i32::MAX as u32))
    }
    /// This preforms the square root of f32s using newton's method
    pub fn sqrt(val: f32) -> f32{
        let mut sqrt = val;
        let mut prev = 0.0;

        while Self::abs(sqrt - prev) > 0.0000000000001{
            prev = sqrt;
            sqrt = Self::sqrt_rec(sqrt, val);
        }

        sqrt
    }

    fn sqrt_rec(val: f32, n: f32) -> f32{
        0.5 * (val + (n / val))
    }

    pub fn sincos(val: T) -> (T, T) {
        let mut val_f32: f32 = val.into();

        let mut cos = 0.0;
        let mut sin = 0.0;

        // shift into range
        let mut shifted = if val_f32 < 0.0 {
            ((val_f32 - FRAC_PI_4) % FRAC_PI_2) + FRAC_PI_4
        }else {
            ((val_f32 + FRAC_PI_4) % FRAC_PI_2) - FRAC_PI_4
        };
        
        if (Self::abs(val_f32) % FRAC_PI_2) < FRAC_PI_4{
            shifted = -shifted
        }

        let calculated = Self::cos_appox_2nd_order(shifted);
        let other = Self::sqrt(1.0 - (calculated * calculated));

        let y = (((4.0 * val_f32) * FRAC_1_PI) + 4.0) % 8.0;
        dbg!(y);

        if ((y + 1.0) % 4.0) < 2.0{
            cos = calculated;
            sin = other;
        }else {
            sin = calculated;
            cos = other;
        }

        if ((y + 1.0) % 8.0) < 4.0{
            sin = -sin;
        }

        if ((y + 3.0) % 8.0) < 4.0{
            cos = -cos;
        }

        (sin.into(), cos.into())
    }

    pub fn sin(val: T) -> T {
        (val.into() * 0.987862 - 0.155271 * (val * val * val).into() + 0.00564312 * (val * val * val * val * val).into()).into()
    }

    pub fn cos(val: T) -> T {
        (1.0 + (val * val).into() * (-4.564684e-9 + (val * val).into() * 3.1372154e-18)).into()
    }


    /// Preforms e^self
    pub fn exp(&mut self) -> T {

        self.re
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