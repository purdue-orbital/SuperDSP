use crate::math::builder::WorkflowBuilder;
use crate::math::ElementParameter;

#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

pub trait Element: ElementClone + Send {
    #[cfg(feature = "ui")]
    /// This will run to setup the window
    fn build_window(&mut self, win_builder: &mut WindowBuilder);
    
    /// This is the main workflow setup. This runs before
    fn init(&mut self, builder: &mut WorkflowBuilder);

    /// This will run if HALT is set to "true"
    fn run(&mut self, samples: &ElementParameter);

    /// This tells the pipeline builder to stop and run the "run" function. You should only do this if you absolutely
    /// need to
    fn halt(&self) -> bool;
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