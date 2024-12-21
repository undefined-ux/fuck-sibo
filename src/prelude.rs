pub use crate::error::SiboError;
pub use crate::model::{UserInformation, SchoolInformation, ClassInformation, Article};
pub use crate::{login, search_school, get_classes, get_articles};

pub type Result<T> = std::result::Result<T, SiboError>;

