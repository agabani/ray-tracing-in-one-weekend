pub struct Compute<T> {
    computes_tx: std::collections::HashMap<usize, std::sync::mpsc::Sender<Option<T>>>,
    join_handles: Vec<Option<std::thread::JoinHandle<()>>>,
}

impl<T> Compute<T> {
    pub fn new<F, R>(
        threads: usize,
        function: F,
    ) -> (Self, std::sync::mpsc::Receiver<ComputeResult<T, R>>)
    where
        F: FnOnce(&T) -> R,
        F: Send + 'static,
        F: Copy,
        T: Send + 'static,
        R: Send + 'static,
    {
        let (orchestration_tx, orchestration_rx) = std::sync::mpsc::channel();

        let mut computes_tx = std::collections::HashMap::with_capacity(threads);
        let mut join_handles = Vec::with_capacity(threads);

        for id in 0..threads {
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
