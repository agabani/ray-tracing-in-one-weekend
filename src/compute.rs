use std::io::Write;

pub struct Compute<T> {
    computes_tx: std::collections::HashMap<usize, std::sync::mpsc::Sender<Option<T>>>,
    join_handles: Vec<Option<std::thread::JoinHandle<()>>>,
}

impl<T> Compute<T> {
    pub fn new<F, R>(
        mut functions: Vec<F>,
    ) -> (Self, std::sync::mpsc::Receiver<ComputeResult<T, R>>)
    where
        F: Fn(&T) -> R,
        F: Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let (orchestration_tx, orchestration_rx) = std::sync::mpsc::channel();

        let mut computes_tx = std::collections::HashMap::with_capacity(functions.len());
        let mut join_handles = Vec::with_capacity(functions.len());

        while let Some(function) = functions.pop() {
            let id = join_handles.len();

            let orchestrator_tx = orchestration_tx.clone();
            let (compute_tx, compute_rx) = std::sync::mpsc::channel();
            computes_tx.insert(id, compute_tx);

            let join_handle = std::thread::spawn(move || {
                for task in compute_rx {
                    if let Some(task) = task {
                        let result = function(&task);

                        orchestrator_tx
                            .send(ComputeResult::new(id, task, result))
                            .unwrap();
                    } else {
                        return;
                    }
                }
            });
            join_handles.push(Some(join_handle));
        }

        (
            Self {
                computes_tx,
                join_handles,
            },
            orchestration_rx,
        )
    }

    pub fn compute(&self, instance: usize, task: T) {
        let compute_tx = self.computes_tx.get(&instance).unwrap();
        compute_tx.send(Some(task)).unwrap();
    }

    pub fn compute_many(&self, jobs: &mut Vec<T>) {
        for compute_tx in self.computes_tx.values() {
            if let Some(job) = jobs.pop() {
                compute_tx.send(Some(job)).unwrap();
            }
        }
    }

    pub fn compute_all<R, F, A>(
        &self,
        receiver: &std::sync::mpsc::Receiver<ComputeResult<T, R>>,
        mut jobs: Vec<T>,
        function: F,
        mut accumulator: A,
    ) -> A
    where
        F: FnOnce(A, &T, R) -> A,
        F: Copy,
        R: Clone,
    {
        let mut processed = 0;
        let total = jobs.len();

        self.compute_many(&mut jobs);

        for result in receiver {
            processed += 1;

            std::io::stderr()
                .write_all(format!("{}/{}\n", processed, total).as_bytes())
                .unwrap();

            let id = result.id();
            accumulator = function(accumulator, result.task(), result.result().clone());

            if processed < total {
                if let Some(pixel) = jobs.pop() {
                    self.compute(id, pixel);
                }
            } else {
                return accumulator;
            }
        }

        accumulator
    }
}

impl<T> Drop for Compute<T> {
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

pub struct ComputeResult<T, R> {
    id: usize,
    task: T,
    result: R,
}

impl<T, R> ComputeResult<T, R> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn task(&self) -> &T {
        &self.task
    }

    pub fn result(&self) -> &R {
        &self.result
    }
}

impl<T, R> ComputeResult<T, R> {
    pub fn new(id: usize, task: T, result: R) -> Self {
        Self { id, task, result }
    }
}
