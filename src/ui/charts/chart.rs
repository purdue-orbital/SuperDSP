use eframe::egui::Ui;

pub trait Chart: ChartClone + Send + Sync {
    /// This will update the chart each frame
    fn update(&self, ui: &mut Ui);
}

pub trait ChartClone {
    fn clone_box(&self) -> Box<dyn Chart>;
}

impl<T> ChartClone for T
    where
        T: 'static + Chart + Clone,
{
    fn clone_box(&self) -> Box<dyn Chart> {
        Box::new(self.clone())
    }
}