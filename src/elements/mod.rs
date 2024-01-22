pub mod element;
pub mod builder;
pub mod signal_generator;
#[cfg(feature = "ui")]
pub mod time_chart;
#[cfg(feature = "ui")]
pub mod constellation_chart;

#[cfg(feature = "ui")]
pub mod waterfall_chart;
mod parts;
//mod merger_element;