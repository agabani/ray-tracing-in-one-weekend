use crate::color::Color;
use crate::pixel::Pixel;

pub struct Compute {
    computes_tx: std::collections::HashMap<usize, std::sync::mpsc::Sender<Option<Pixel>>>,
    join_handles: Vec<Option<std::thread::JoinHandle<()>>>,
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
                    if let Some(task) = task {
                        let color = function(&task);

                        orchestrator_tx
                            .send(ComputeResult::new(instance, task, color))
                            .unwrap();
                    } else {
                        return;
                    }
                }
            });
            join_handles.push(Some(join_handle));
        }

        Self {
            computes_tx,
            join_handles,
        }
    }

    pub fn compute(&self, instance: usize, pixel: Pixel) {
        let compute_tx = self.computes_tx.get(&instance).unwrap();
        compute_tx.send(Some(pixel)).unwrap();
    }

    pub fn compute_many(&self, jobs: &mut Vec<Pixel>) {
        for compute_tx in self.computes_tx.values() {
            if let Some(job) = jobs.pop() {
                compute_tx.send(Some(job)).unwrap();
            }
        }
    }
}

impl Drop for Compute {
    fn drop(&mut self) {
        for computes_tx in self.computes_tx.values() {
            computes_tx.send(None).unwrap();
        }

        for join_handle in self.join_handles.iter_mut() {
            if let Some(join_handle) = join_handle.take() {
                join_handle.join().unwrap();
            }
        }
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
