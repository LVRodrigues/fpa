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

/// Largest page safe for every supported page size (up to 50).
pub(crate) const MAX_PAGE: u64 = i64::MAX as u64 / 50 + 1;

/// Deserialize a page index and reject values that cannot safely be used as a
/// database offset. Missing values keep the documented default.
pub(crate) fn deserialize_page<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<u64>::deserialize(deserializer)?;
    if value.is_some_and(|page| {
        page.checked_sub(1)
            .and_then(|index| index.checked_mul(50))
            .is_none_or(|offset| offset > i64::MAX as u64)
    }) {
        return Err(D::Error::custom(format!(
            "page must be between 1 and {MAX_PAGE}"
        )));
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
    #[param(minimum = 1, maximum = 184467440737095517_u64, default = 1)]
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

#[cfg(test)]
mod tests {
    use axum::{extract::Query, http::Uri};

    use super::PageParams;

    fn parse(query: &str) -> Result<Query<PageParams>, axum::extract::rejection::QueryRejection> {
        let uri: Uri = format!("/?{query}").parse().expect("test URI is valid");
        Query::try_from_uri(&uri)
    }

    #[test]
    fn pagination_defaults_when_parameters_are_missing() {
        let Query(params) = parse("").expect("empty query should use defaults");

        assert_eq!(params.page(), 1);
        assert_eq!(params.size(), 10);
    }

    #[test]
    fn pagination_accepts_documented_boundaries() {
        let Query(params) = parse("page=1&size=50").expect("boundary values should be valid");

        assert_eq!(params.page(), 1);
        assert_eq!(params.size(), 50);
    }

    #[test]
    fn pagination_rejects_page_zero() {
        assert!(parse("page=0").is_err());
    }

    #[test]
    fn pagination_rejects_page_larger_than_safe_offset_range() {
        assert!(parse("page=9223372036854775808").is_err());
    }

    #[test]
    fn pagination_bounds_prevent_offset_overflow() {
        let Query(params) = parse(&format!("page={}&size=50", super::MAX_PAGE)).unwrap();
        assert!((params.page() - 1).checked_mul(params.size()).unwrap() <= i64::MAX as u64);
        assert!(parse(&format!("page={}&size=50", super::MAX_PAGE + 1)).is_err());
        assert!(parse("page=9223372036854775807&size=50").is_err());
    }

    #[test]
    fn pagination_rejects_size_outside_one_to_fifty() {
        assert!(parse("size=0").is_err());
        assert!(parse("size=51").is_err());
    }
}
