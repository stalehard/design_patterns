#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Stopped,
    Failed,
    Completed,
}

// The "Implementor" trait. This is the implementation part of the bridge.
trait JobImpl {
    fn run(&mut self) -> String;
    fn stop(&mut self) -> String;
    fn status(&self) -> &JobStatus;
}

// A concrete "Implementor".
pub struct SingleJob {
    id: u32,
    status: JobStatus,
}

impl SingleJob {
    pub fn new(id: u32) -> Self {
        SingleJob {
            id,
            status: JobStatus::Pending,
        }
    }
}

impl JobImpl for SingleJob {
    fn run(&mut self) -> String {
        self.status = JobStatus::Running;
        format!("Running single job {}", self.id)
    }

    fn stop(&mut self) -> String {
        self.status = JobStatus::Stopped;
        format!("Stopping single job {}", self.id)
    }

    fn status(&self) -> &JobStatus {
        &self.status
    }
}

// Another concrete "Implementor".
pub struct MultipleJob {
    jobs: Vec<Box<dyn JobImpl>>,
    status: JobStatus,
}

impl MultipleJob {
    pub fn new(jobs: Vec<Box<dyn JobImpl>>) -> Self {
        MultipleJob {
            jobs,
            status: JobStatus::Pending,
        }
    }
}

impl JobImpl for MultipleJob {
    fn run(&mut self) -> String {
        self.status = JobStatus::Running;
        let results: Vec<String> = self.jobs.iter_mut().map(|job| job.run()).collect();
        format!("Running multiple jobs:\n{}", results.join("\n"))
    }

    fn stop(&mut self) -> String {
        self.status = JobStatus::Stopped;
        let results: Vec<String> = self.jobs.iter_mut().map(|job| job.stop()).collect();
        format!("Stopping multiple jobs:\n{}", results.join("\n"))
    }

    fn status(&self) -> &JobStatus {
        // In a real-world scenario, this might check the status of all sub-jobs.
        &self.status
    }
}

// The "Abstraction". This is the public-facing part of the bridge.
pub struct Job {
    implementation: Box<dyn JobImpl>,
}

impl Job {
    pub fn new(implementation: Box<dyn JobImpl>) -> Self {
        Job { implementation }
    }

    pub fn run(&mut self) -> String {
        self.implementation.run()
    }

    pub fn stop(&mut self) -> String {
        self.implementation.stop()
    }

    pub fn status(&self) -> &JobStatus {
        self.implementation.status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_job_lifecycle() {
        let single_job_impl = SingleJob::new(1);
        assert_eq!(*single_job_impl.status(), JobStatus::Pending);

        let mut job = Job::new(Box::new(single_job_impl));
        assert_eq!(*job.status(), JobStatus::Pending);

        assert_eq!(job.run(), "Running single job 1");
        assert_eq!(*job.status(), JobStatus::Running);

        assert_eq!(job.stop(), "Stopping single job 1");
        assert_eq!(*job.status(), JobStatus::Stopped);
    }

    #[test]
    fn test_multiple_job_lifecycle() {
        let sub_job1 = Box::new(SingleJob::new(10));
        let sub_job2 = Box::new(SingleJob::new(11));
        let multiple_job_impl = MultipleJob::new(vec![sub_job1, sub_job2]);
        assert_eq!(*multiple_job_impl.status(), JobStatus::Pending);

        let mut job = Job::new(Box::new(multiple_job_impl));
        assert_eq!(*job.status(), JobStatus::Pending);

        let expected_run_output =
            "Running multiple jobs:\nRunning single job 10\nRunning single job 11";
        assert_eq!(job.run(), expected_run_output);
        assert_eq!(*job.status(), JobStatus::Running);

        let expected_stop_output =
            "Stopping multiple jobs:\nStopping single job 10\nStopping single job 11";
        assert_eq!(job.stop(), expected_stop_output);
        assert_eq!(*job.status(), JobStatus::Stopped);
    }

    #[test]
    fn test_nested_multiple_job() {
        let sub_job1 = Box::new(SingleJob::new(100));
        let sub_job2 = Box::new(SingleJob::new(101));
        let inner_multiple_job = Box::new(MultipleJob::new(vec![sub_job1, sub_job2]));

        let sub_job3 = Box::new(SingleJob::new(200));

        let outer_multiple_job_impl = MultipleJob::new(vec![inner_multiple_job, sub_job3]);
        assert_eq!(*outer_multiple_job_impl.status(), JobStatus::Pending);

        let mut job = Job::new(Box::new(outer_multiple_job_impl));

        let expected_run_output = "Running multiple jobs:\nRunning multiple jobs:\nRunning single job 100\nRunning single job 101\nRunning single job 200";
        assert_eq!(job.run(), expected_run_output);
        assert_eq!(*job.status(), JobStatus::Running);

        let expected_stop_output = "Stopping multiple jobs:\nStopping multiple jobs:\nStopping single job 100\nStopping single job 101\nStopping single job 200";
        assert_eq!(job.stop(), expected_stop_output);
        assert_eq!(*job.status(), JobStatus::Stopped);
    }
}
