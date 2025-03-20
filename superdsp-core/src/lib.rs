#![no_std]

pub mod stages;

extern crate core;

// TODO: Get rid of alloc. I want this to work on microcontrollers
extern crate alloc;

use crate::stages::fsk::FSKSettings;
use crate::stages::packet_detection::PacketDetectionSettingsF32;
use crate::stages::wave_gen::WaveGenInformation;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::{Ref, RefCell, RefMut};
use core::f32::consts::PI;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use nalgebra::DMatrix;
use num::Complex;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Schedule {
    Startup,
    Update,
}

pub trait Stage {
    fn invoke(&mut self, args: &mut BTreeMap<TypeId, RefCell<Box<dyn Any>>>);
}

pub type StoredStage = Box<dyn Stage>;

trait IntoStage<Input> {
    type Stage: Stage;
    fn into_stage(self) -> Self::Stage;
}

trait StageParam {
    type Item<'new>;
    fn retrieve<'r>(resources: &'r BTreeMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r>;
}

pub struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}

pub struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

impl<'res, T: 'static> StageParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(resources: &'r BTreeMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r> {
        Res {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

impl<'res, T: 'static> StageParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'r>(
        resources: &'r BTreeMap<TypeId, RefCell<Box<(dyn Any + 'static)>>>,
    ) -> Self::Item<'r> {
        ResMut {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_stage {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F: FnMut($($params),*), $($params: StageParam),*> Stage for FunctionStage<($($params),*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as StageParam>::Item<'b>),*)
        {
            fn invoke(&mut self, resources: &mut BTreeMap<TypeId, RefCell<Box<dyn Any>>>) {
                fn call_inner<$($params),*>(
                        mut f: impl FnMut($($params),*),
                    $(
                        $params: $params,
                    )*
                ) {
                    f($($params),*)
                }

                $(
                    let $params = $params::retrieve(resources);
                )*

                call_inner(&mut self.f, $($params),*)
            }
        }

        impl<F: FnMut($($params),*), $($params: StageParam),*> IntoStage<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as StageParam>::Item<'b>),*)
        {
            type Stage = FunctionStage<($($params),*), Self>;

            fn into_stage(self) -> Self::Stage {
                FunctionStage{
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_stage!();
impl_stage!(A);
impl_stage!(A, B);
impl_stage!(A, B, C);
impl_stage!(A, B, C, D);
impl_stage!(A, B, C, D, E);
impl_stage!(A, B, C, D, E, G);
impl_stage!(A, B, C, D, E, G, H);
impl_stage!(A, B, C, D, E, G, H, I);
impl_stage!(A, B, C, D, E, G, H, I, J);
impl_stage!(A, B, C, D, E, G, H, I, J, K);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, R);

pub struct Scheduler {
    startup_stages: Option<Vec<StoredStage>>,
    update_stages: Option<Vec<StoredStage>>,

    resources: Option<BTreeMap<TypeId, RefCell<Box<dyn Any>>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            startup_stages: Some(vec![]),
            update_stages: Some(vec![]),
            resources: Some(BTreeMap::new()),
        }
    }

    /// Adds a stage to the scheduler based on the specified schedule.
    ///
    /// # Arguments
    ///
    /// * `schedule` - A `Schedule` enum indicating whether the stage should be added to the startup or update stages.
    /// * `stage` - An implementation of `IntoStage<I, Stage = S>` that can be converted into a `Stage`.
    pub fn add_stage<I, S: Stage + 'static>(
        &mut self,
        schedule: Schedule,
        stage: impl IntoStage<I, Stage = S>,
    ) {
        match schedule {
            Schedule::Startup => {
                self.startup_stages
                    .as_mut()
                    .unwrap()
                    .push(Box::new(stage.into_stage()));
            }
            Schedule::Update => {
                self.update_stages
                    .as_mut()
                    .unwrap()
                    .push(Box::new(stage.into_stage()));
            }
        }
    }

    /// Adds a plugin to the scheduler.
    ///
    /// # Arguments
    ///
    /// * `plugin` - A closure that takes a mutable reference to the Scheduler and performs some operations on it.
    pub fn add_plugin(&mut self, plugin: impl Fn(&mut Scheduler)) {
        plugin(self);
    }

    /// Adds a resource to the scheduler.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be added, which must have a static lifetime.
    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn build(&mut self) -> Runner {
        let mut s_stages = self.startup_stages.take().unwrap();

        for stage in s_stages.iter_mut() {
            stage.invoke(&mut self.resources.as_mut().unwrap());
        }

        Runner {
            stages: self.update_stages.take().unwrap(),
            resources: self.resources.take().unwrap(),
        }
    }
}

pub struct Runner {
    stages: Vec<StoredStage>,
    resources: BTreeMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl Runner {
    pub fn run(&mut self) {
        loop {
            for stage in self.stages.iter_mut() {
                stage.invoke(&mut self.resources);
            }
        }
    }
}

pub struct FunctionStage<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

#[derive(Default)]
pub struct DspInformation {
    pub carrier_frequency: f32,

    pub sample_rate: f32,

    pub gain: f32,

    pub taps: usize,
}

impl DspInformation {
    pub fn create_wave_gen_settings(&self) -> WaveGenInformation {
        WaveGenInformation {
            c_radians: 0.0,
            radians_a_sample: 2.0 * PI * self.carrier_frequency / self.sample_rate,
        }
    }

    pub fn create_fsk_settings(&self) -> FSKSettings {
        FSKSettings {
            channel_0: self.taps
                - ((self.taps as f32 / self.sample_rate) * self.carrier_frequency) as usize,
            channel_1: ((self.taps as f32 / self.sample_rate) * self.carrier_frequency) as usize,

            radians_a_sample_c0: -2.0 * PI * self.carrier_frequency / self.sample_rate,
            radians_a_sample_c1: 2.0 * PI * self.carrier_frequency / self.sample_rate,

            c_radian_c0: 0.0,
            c_radian_c1: 0.0,
        }
    }
}

pub fn DSPCore(scheduler: &mut Scheduler) {
    scheduler.add_resource(DspInformation {
        ..Default::default()
    });

    scheduler.add_resource(Vec::new() as Vec<Complex<f32>>);
    scheduler.add_resource(Vec::new() as Vec<u8>);
    scheduler.add_resource(Vec::new() as Vec<f32>);
    scheduler.add_resource(0u8);
    scheduler.add_resource(0isize);
    scheduler.add_resource(0usize);
    scheduler.add_resource(0f32);
    scheduler.add_resource(Complex::<f32>::default());

    scheduler.add_resource(PacketDetectionSettingsF32 {
        matrix: DMatrix::zeros(1, 1),
        threshold: Default::default(),
        buffer: DMatrix::zeros(1, 1),
    });

    scheduler.add_resource(FSKSettings {
        ..Default::default()
    });
}
