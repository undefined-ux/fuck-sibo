pub use crate::error::SiboError;
pub use crate::model::{Article, ClassInformation, SchoolInformation, UserInformation};
pub use crate::{get_articles, get_classes, login, search_school};

pub type SiboResult<T> = Result<T, SiboError>;
