pub struct Data<'a>{
    pub(crate) f32_arrays: Vec<&'a mut [f32]>
}

pub trait CPUOperation{
    fn run(&mut self, data: &mut Data);
}

pub struct ElementwiseMultiplyF32;
impl CPUOperation for ElementwiseMultiplyF32{
    fn run(&mut self, data: &mut Data) {

        let mb: &mut Vec<&mut [f32]> = data.f32_arrays.as_mut();

        // run
        for index in 0..mb.get(0).unwrap().len(){

            let val2 = mb.get(1).unwrap().get(index).unwrap();
            let val1 = mb.get(0).unwrap().get(index).unwrap();

            mb[1][index] = *val1 * *val2;
        }
    }
}