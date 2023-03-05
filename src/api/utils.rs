use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

// It's needed because Some(None) serializes to null
#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateOption<T> {
    None,
    Some(T),
}

pub use UpdateOption::None as UNone;
pub use UpdateOption::Some as USome;

impl<T> From<Option<T>> for UpdateOption<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Self::Some(t),
            None => Self::None,
        }
    }
}

impl<T> From<UpdateOption<T>> for Option<T> {
    fn from(value: UpdateOption<T>) -> Self {
        match value {
            UpdateOption::Some(t) => Self::Some(t),
            UpdateOption::None => Self::None,
        }
    }
}

impl<T> Clone for UpdateOption<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Some(t) => Self::Some(t.clone()),
        }
    }
}

impl<T> UpdateOption<T> {
    pub fn option(self) -> Option<T> {
        self.into()
    }

    pub fn option_ref(&self) -> Option<&T> {
        match self {
            UNone => None,
            USome(t) => Some(t),
        }
    }

    pub fn option_mut(&mut self) -> Option<&mut T> {
        match self {
            UNone => None,
            USome(t) => Some(t),
        }
    }
}
