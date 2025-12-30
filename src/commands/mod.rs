pub mod delete;
pub mod link;
pub mod list;
pub mod save;

use crate::error::KpvError;
use crate::storage::Storage;

pub(crate) trait Execute<R> {
    fn execute(&self, storage: &impl Storage) -> Result<R, KpvError>;
}
