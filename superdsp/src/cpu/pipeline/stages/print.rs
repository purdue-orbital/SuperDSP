use crate::cpu::pipeline::stages::{Data, DataKind, LastStage, Stage};
use crate::cpu::pipeline::PipelineSettings;

#[derive(Default)]
pub struct Print {
    empty_data: Data,
}

impl<I, O> Stage<I, O> for Print {
    fn configure(&mut self, data: &mut PipelineSettings) {
        data.prev_stage_output = DataKind::F64;
    }

    fn process(&mut self, data: &Data, output: &mut Data) {
        let d = data.get_f64_data();
        output.set_f64_data(d.clone());
    }

    fn get_output_data_type(&self) -> DataKind {
        DataKind::F64
    }

    fn get_input_data_type(&self) -> DataKind {
        DataKind::F64
    }
}

impl<I, O: Send + Sync> LastStage<I, O> for Print {}