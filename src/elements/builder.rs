use std::thread::spawn;

use crate::elements::element::Element;
use crate::math::prelude::*;
#[cfg(feature = "ui")]
use crate::ui::charts::builder::WindowBuilder;

pub struct Pipeline {
    sps: usize,

    #[cfg(feature = "ui")]
    window: WindowBuilder,

    pipeline: Option<Vec<PipeSegment>>,
}

impl Pipeline {
    /// ***THIS WILL HALT THE THREAD AND IT MUST BE THE MAIN THREAD!*** This is because of a limitation set by egui
    pub fn run(&mut self) {
        let mut elements = self.pipeline.take();

        spawn(move || {
            loop {
                for x in elements.as_mut().unwrap().as_mut_slice() {
                    x.run()
                }
            }
        });


        #[cfg(feature = "ui")]
        self.window.build()
    }
}

struct PipeSegment {
    element: Option<Box<dyn Element>>,

    element_input: Option<ElementParameter>,

    workflow: Option<Workflow>,
}

impl PipeSegment {
    pub fn run(&mut self) {
        if self.element.is_some() {
            self.element.as_mut().unwrap().run(self.element_input.as_ref().unwrap());
        } else {
            self.workflow.as_mut().unwrap().run()
        }
    }
}

pub struct PipelineBuilder {
    pipeline: Option<Vec<PipeSegment>>,

    current_workflow: WorkflowBuilder,

    buffer: ElementParameter,

    #[cfg(feature = "ui")]
    window_builder: Option<WindowBuilder>,
}

impl PipelineBuilder {
    pub fn new() -> PipelineBuilder {
        PipelineBuilder {
            pipeline: Some(vec![]),
            current_workflow: WorkflowBuilder::default(),
            buffer: ElementParameter::new_f32_array(&[0.0]),

            #[cfg(feature = "ui")]
            window_builder: Some(WindowBuilder::new()),
        }
    }

    pub fn add<T: Element + 'static>(&mut self, mut element: T) {

        // initialize
        element.init(&mut self.current_workflow, &mut self.buffer);

        #[cfg(feature = "ui")]
        element.build_window(&mut self.window_builder.as_mut().unwrap());

        // check if workflow halts
        if element.halt() {

            // if workflow halts, check if workflow has more than one operation
            if self.current_workflow.num_operations > 0 {
                self.pipeline.as_mut().unwrap().push(PipeSegment {
                    element: None,
                    element_input: None,
                    workflow: Some(self.current_workflow.build()),
                });

                self.current_workflow = WorkflowBuilder::default();
            }

            // add element
            self.pipeline.as_mut().unwrap().push(PipeSegment {
                element: Some(Box::new(element)),
                element_input: Some(self.buffer.clone()),
                workflow: None,
            })
        }
    }

    /// This will setup pipeline to run creating a sender, receiver, and pipeline
    pub fn build(&mut self, sps: usize) -> Pipeline {
        Pipeline {
            sps,

            #[cfg(feature = "ui")]
            window: self.window_builder.take().unwrap(),
            pipeline: Some(self.pipeline.take().unwrap()),
        }
    }
}