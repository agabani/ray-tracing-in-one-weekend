use crate::color::Color;
use crate::pixel::Pixel;

pub struct Compute {
    computes_tx: std::collections::HashMap<usize, std::sync::mpsc::Sender<Pixel>>,
    join_handles: Vec<std::thread::JoinHandle<()>>,
}

impl Compute {
    pub fn new<F>(
        instances: usize,
        orchestrator_tx: std::sync::mpsc::Sender<ComputeResult>,
        function: F,
    ) -> Self
    where
        F: FnOnce(&Pixel) -> Color,
        F: Send + 'static,
        F: Copy,
    {
        let mut computes_tx = std::collections::HashMap::with_capacity(instances);
        let mut join_handles = Vec::with_capacity(instances);

        for instance in 0..instances {
            let orchestrator_tx = orchestrator_tx.clone();
            let (compute_tx, compute_rx) = std::sync::mpsc::channel();
            computes_tx.insert(instance, compute_tx);

            let join_handle = std::thread::spawn(move || {
                for task in compute_rx {
                    let color = function(&task);

                    orchestrator_tx
                        .send(ComputeResult::new(instance, task, color))
                        .unwrap();
                }
            });
            join_handles.push(join_handle);
        }

        Self {
            computes_tx,
            join_handles,
        }
    }

    pub fn computes_tx(&self) -> &std::collections::HashMap<usize, std::sync::mpsc::Sender<Pixel>> {
        &self.computes_tx
    }
}

pub struct ComputeResult {
    id: usize,
    pixel: Pixel,
    color: Color,
}

impl ComputeResult {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn pixel(&self) -> &Pixel {
        &self.pixel
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl ComputeResult {
    pub fn new(id: usize, pixel: Pixel, color: Color) -> Self {
        Self { id, pixel, color }
    }
}
