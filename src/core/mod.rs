pub mod delete;
pub mod link;
pub mod list;
pub mod save;

use crate::error::KpvError;
use crate::storage::Storage;

#[cfg(test)]
pub(crate) mod test_support;

pub(crate) trait Execute<R> {
    fn execute(&self, storage: &impl Storage) -> Result<R, KpvError>;
}
