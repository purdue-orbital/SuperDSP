use superdsp_core::{DSPCore, DspInformation, Res, ResMut, Scheduler, Stage};
use superdsp_core::Schedule::{Startup, Update};

fn hello_world_stage() {
    println!("Hello, world!");
}

fn get_information_stage_example(radio_information: Res<DspInformation>, ) {
    let f = radio_information.carrier_frequency;
    let gain = radio_information.gain;
    
    
    println!("Frequency: {}, Gain: {}", f, gain);
}

fn set_information_stage_example(mut radio_information: ResMut<DspInformation>) {
    radio_information.gain = 10.0;
    radio_information.carrier_frequency = 440.0;
    radio_information.taps = 10;
}

fn get_demo_resource(demo: Res<ResourceExample>){
    println!("{}", demo.name);
}

pub struct ResourceExample{
    pub name: String,
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_plugin(DSPCore::<16>);
    
    s.add_resource(ResourceExample{
        name: String::from("Hello, world"),
    });
    
    s.add_stage(Update, hello_world_stage);
    s.add_stage(Update, get_information_stage_example);
    s.add_stage(Update, get_demo_resource);
    
    s.add_stage(Startup, set_information_stage_example);
    
    s.setup();
    
    loop{
        s.run();
    }
}