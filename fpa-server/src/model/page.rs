use serde::{de::Error as _, Deserialize, Deserializer, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};

/// Page selected.
#[derive(Debug, Clone, Serialize, ToSchema, ToResponse)]
pub struct Page<T: ToSchema> {
    /// Total of pages.
    pub pages: u64,
    /// Index of this page.
    pub index: u64,
    /// Records in this page.
    pub size: u64,
    /// Total of records.
    pub records: u64,
    /// List of records.
    pub items: Vec<T>,
}

/// Deserialize a page index and reject values that cannot safely be used as a
/// database offset. Missing values keep the documented default.
pub(crate) fn deserialize_page<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<u64>::deserialize(deserializer)?;
    if value.is_some_and(|page| page == 0 || page > i64::MAX as u64) {
        return Err(D::Error::custom("page must be between 1 and i64::MAX"));
    }
    Ok(value)
}

/// Deserialize a page size and enforce the public API limit.
pub(crate) fn deserialize_page_size<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<u64>::deserialize(deserializer)?;
    if value.is_some_and(|size| !(1..=50).contains(&size)) {
        return Err(D::Error::custom("size must be between 1 and 50"));
    }
    Ok(value)
}

impl<T: ToSchema> Page<T> {
    pub fn new() -> Self {
        Self {
            pages: 0,
            index: 0,
            size: 0,
            records: 0,
            items: Vec::<T>::new(),
        }
    }
}

/// Page select params.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PageParams {
    /// Index of page to select.
    #[param(minimum = 1, default = 1)]
    #[serde(default, deserialize_with = "deserialize_page")]
    page: Option<u64>,
    /// Page's size (records).
    #[param(minimum = 1, maximum = 50, default = 10)]
    #[serde(default, deserialize_with = "deserialize_page_size")]
    size: Option<u64>,
    /// Filter by name.
    #[param()]
    name: Option<String>,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            size: Some(10),
            name: Some(String::new()),
        }
    }
}

impl PageParams {
    pub fn page(&self) -> u64 {
        match self.page {
            Some(v) => v,
            None => 1,
        }
    }

    pub fn size(&self) -> u64 {
        match self.size {
            Some(v) => v,
            None => 10,
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
}
