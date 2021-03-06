// https://developer.apple.com/documentation/apple_search_ads/keywordinsights

use serde::Deserialize;

use crate::v3::objects::keyword_bid_recommendation::KeywordBidRecommendation;

#[derive(Deserialize, Debug, Clone)]
pub struct KeywordInsights {
    #[serde(rename = "bidRecommendation")]
    pub bid_recommendation: KeywordBidRecommendation,
}
