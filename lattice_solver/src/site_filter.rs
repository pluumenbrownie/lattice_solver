use crate::*;

#[derive(Clone, Debug)]
pub struct SiteFilter {
    pub(crate) wrapped: Vec<OxygenIndex>,
}

impl SiteFilter {
    #[must_use]
    pub const fn empty() -> Self {
        Self { wrapped: vec![] }
    }
}
