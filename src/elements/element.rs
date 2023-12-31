use num_complex::Complex;

pub trait Element: ElementClone + Send{
    /// This will run during the build process
    fn init(&mut self);
    
    /// This will run during operations
    fn run(&mut self, samples: &mut [Complex<f32>]);
}

pub trait ElementClone {
    fn clone_box(&self) -> Box<dyn Element>;
}

impl<T> ElementClone for T
    where
        T: 'static + Element + Clone,
{
    fn clone_box(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }
}