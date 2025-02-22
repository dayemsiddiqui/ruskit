mod test_job;

pub use test_job::TestJob;

use crate::framework::queue::worker::register_job;

pub fn register_jobs() {
    register_job::<TestJob>();
} 