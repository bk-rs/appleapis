// https://developer.apple.com/documentation/apple_search_ads/reportingsearchterm

use serde::Deserialize;

use crate::v3::types::region::Region;

#[derive(Deserialize, Debug, Clone)]
pub struct ReportingSearchTerm {
    // TODO
    #[serde(rename = "searchTermText")]
    pub search_term_text: Option<String>,

    #[serde(rename = "searchTermSource")]
    pub search_term_source: SearchTermSource,

    #[serde(rename = "keywordId")]
    pub keyword_id: Option<u64>,

    pub keyword: String,

    #[serde(rename = "matchType")]
    pub match_type: ReportingSearchTermMatchType,

    #[serde(rename = "adGroupId")]
    pub ad_group_id: u64,

    #[serde(rename = "adGroupName")]
    pub ad_group_name: String,

    #[serde(rename = "countryOrRegion")]
    pub country_or_region: Region,
}

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum SearchTermSource {
    #[allow(clippy::upper_case_acronyms)]
    AUTO,
    #[allow(clippy::upper_case_acronyms)]
    TARGETED,
}

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum ReportingSearchTermMatchType {
    #[allow(clippy::upper_case_acronyms)]
    AUTO,
    #[allow(clippy::upper_case_acronyms)]
    EXACT,
    #[allow(clippy::upper_case_acronyms)]
    BROAD,
}
