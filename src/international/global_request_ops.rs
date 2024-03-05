use crate::{constants::global_requests::{MAX_CLAIM_WORK_REQUEST_DISTANCE, MAX_WORK_REQUEST_DISTANCE}, memory::global_requests::{WorkRequest}};

pub struct GlobalRequestOps;

impl GlobalRequestOps {
    /// Would be nice for this method to work for all request types that have a property "abandon"
    pub fn is_abandoned(request: &WorkRequest) -> bool {
        match request.abandon {
            None => {
                return true;
            }
            Some(abandon) => {
               if abandon > 0 {
                return true
               }

               false
            }
        }
    }
}