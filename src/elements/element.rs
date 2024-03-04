use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

pub trait Element: ElementClone + Send {
    #[cfg(feature = "ui")]
    /// This will run to set up the window
    fn build_window(&mut self, win_builder: &mut WindowBuilder);

    /// This is the main workflow setup. This runs before build window. Samples is the input and 
    /// output buffer
    fn init(&mut self, builder: &mut WorkflowBuilder, samples: &mut ElementParameter);

    /// This will run if halt is returns "true"
    fn run(&mut self, samples: &mut ElementParameter);

    /// This tells the pipeline builder to stop and run the "run" function. You should only do this 
    /// if you absolutely need to
    fn halt(&self) -> bool;
    
    /// This tells the pipeline during run time that this is the last element in the pipeline and to
    /// stop here 
    fn stop(&self, samples: &mut ElementParameter) -> bool;

    /// This will tell the workflow builder that this block is the top part in a workflow chain 
    /// (like signal generators) and doesn't take an input
    fn is_source(&self) -> bool;
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