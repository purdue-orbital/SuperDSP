use num_complex::Complex;
use crate::elements::element::Element;

pub struct Pipeline {
    sps: usize,

    elements: Vec<Box<dyn Element>>
}

impl Pipeline {
    pub fn run(&mut self, samples: &mut [Complex<f32>]){
        debug_assert_eq!(samples.len(), self.sps);

        for x in self.elements.as_mut_slice(){
            x.run(samples)
        }
    }
}

// pub enum BaseElements{
//     LineChart,
//
// }

pub struct PipelineBuilder {
    elements: Vec<Box<dyn Element>>
}

impl PipelineBuilder {
    pub fn new() -> PipelineBuilder {
        PipelineBuilder {
            elements: vec![]
        }
    }

    // pub fn add_from_base(&mut self, element: BaseElements){
    //     match element {
    //         elements::builder::BaseElements::LineChart => self.elements.,
    //     }
    // }

    pub fn add<T: Element>(&mut self, element: T) -> Box<T>{
        let b = Box::new(element);

        self.elements.push(b.clone_box());

        b
    }

    pub fn build(&mut self, sps: usize) -> Pipeline {
        let mut program = Pipeline {
            sps,
            elements: vec![],
        };

        for x in self.elements.as_mut_slice(){
            x.init();

            program.elements.push(x.clone_box())
        }

        program
    }
}