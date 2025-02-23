pub mod stages;

use crate::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use tokio::task::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::cpu::pipeline::stages::{Data, DataKind, FirstStage, LastStage, Stage};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PipelineType {
    Loop,
    OnSend,
    OnRecv,
}

#[derive(Default)]
pub struct PipelineSettings {
    frequency: Option<f64>,
    sample_rate: Option<f64>,

    sps: Option<usize>,

    num_taps: Option<usize>,

    pub prev_stage_output: DataKind,
}

impl PipelineSettings {
    pub fn set_frequency(&mut self, freq: f64) -> &mut Self {
        self.frequency = Some(freq);
        self
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) -> &mut Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    pub fn set_sps(&mut self, sps: usize) -> &mut Self {
        self.sps = Some(sps);
        self
    }

    pub fn set_num_taps(&mut self, num_taps: usize) -> &mut Self {
        self.num_taps = Some(num_taps);
        self
    }
}

#[derive(Default)]
pub struct PipelineBuilder<I: Into<Data>, O: From<Data>> {
    first_stage: Option<Box<dyn FirstStage<I, O>>>,

    last_stage: Option<Box<dyn LastStage<I, O>>>,

    stages: Vec<Option<Box<dyn Stage<I, O>>>>,
}

impl<I: Clone + Into<Data> + 'static, O: From<Data> + 'static> PipelineBuilder<I, O> {
    pub fn add_stage(&mut self, stage: Box<dyn Stage<I, O>>) -> &mut Self {
        self.stages.push(Some(stage));

        self
    }

    pub fn add_first_stage(&mut self, stage: Box<dyn FirstStage<I, O>>) -> &mut Self {
        self.first_stage = Some(stage);
        self
    }

    pub fn add_last_stage(&mut self, stage: Box<dyn LastStage<I, O>>) -> &mut Self {
        self.last_stage = Some(stage);
        self
    }

    pub async fn build(&mut self, pipeline_type: PipelineType, settings: &mut PipelineSettings) -> Pipeline<I, O> {
        // make sure the required fields are present
        assert!(self.first_stage.is_some());
        assert!(self.last_stage.is_some());

        // configure the interstage connections
        let mut first_stage = self.first_stage.take().unwrap();
        first_stage.configure(settings);

        let (first_in_tx, mut first_in_rx) = tokio::sync::mpsc::channel(16);
        let (out_tx, out_rx) = tokio::sync::mpsc::channel(16);

        spawn(async move {
            loop {
                let data = first_in_rx.recv().await.unwrap();
                let mut out = Data::new(DataKind::Empty);

                first_stage.process(&data, &mut out);
                out_tx.send(out).await.unwrap();
            }
        });
        
        let mut prev_out = out_rx;

        for x in self.stages.iter_mut() {
            let mut in_rx = prev_out;
            let (out_tx, out_rx) = tokio::sync::mpsc::channel(16);

            let mut stage = x.take().unwrap();
            stage.configure(settings);

            // start the interstage threads
            spawn(async move {
                loop {
                    let mut out = Data::new(DataKind::Empty);

                    let data = in_rx.recv().await.unwrap();

                    stage.process(&data, &mut out);

                    out_tx.send(out).await.unwrap();
                }
            });
            
            prev_out = out_rx;
        }


        let mut in_rx = prev_out;
        let (out_tx, mut out_rx) = tokio::sync::mpsc::channel(16);

        let mut last_stage = self.last_stage.take().unwrap();
        last_stage.configure(settings);

        spawn(async move {
            loop {
                let mut out = Data::new(DataKind::Empty);

                let data = in_rx.recv().await.unwrap();

                last_stage.process(&data, &mut out);

                out_tx.send(out).await.unwrap();
            }
        });

        match pipeline_type {
            PipelineType::Loop => {
                // We take both as the last stage isn't used in a loop
                spawn(async move {
                    loop {
                        first_in_tx.send(Data::new(DataKind::Empty)).await.unwrap();
                    }
                });

                // For some fucking reason, this needs to be in its own thread to work as it 
                // implements unreachable_unchecked function
                spawn(async move {
                    loop {
                        out_rx.recv().await.unwrap();
                    }
                });

                Pipeline {
                    first_stage_in: None,
                    last_stage_out: None,

                    phantom: Default::default(),
                    pipeline_type,
                }
            }
            PipelineType::OnSend => {
                spawn(async move {
                    loop {
                        out_rx.recv().await.unwrap();
                    }
                });
                
                // Only send is usable
                Pipeline {
                    first_stage_in: Some(first_in_tx),
                    last_stage_out: None,

                    phantom: Default::default(),
                    pipeline_type,
                }
            }
            PipelineType::OnRecv => {
                // Only recv is usable
                spawn(async move {
                    loop {
                        first_in_tx.send(Data::new(DataKind::Empty)).await.unwrap();
                    }
                });

                Pipeline {
                    first_stage_in: None,
                    last_stage_out: Some(out_rx),

                    phantom: Default::default(),
                    pipeline_type,
                }
            }
        }
    }
}

pub struct Pipeline<I, O> {
    first_stage_in: Option<Sender<Data>>,
    last_stage_out: Option<Receiver<Data>>,

    phantom: PhantomData<(I, O)>,

    pipeline_type: PipelineType,
}
impl<I: Into<Data> + Clone + Send + Sync + Debug + 'static, O: Into<O> + Clone + Send + Sync + 'static + Into<Data> + From<Data>> Pipeline<I, O> {
    pub async fn send(&mut self, data: I) -> anyhow::Result<()> {
        self.first_stage_in.as_ref().unwrap().send(data.into()).await?;

        Ok(())
    }

    pub async fn recv(&mut self) -> anyhow::Result<O> {
        Ok(self.last_stage_out.as_mut().unwrap().recv().await.unwrap().into())
    }
}