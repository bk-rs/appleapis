pub mod get_ad_group_level_reports;
pub mod get_all_campaigns;
pub mod get_campaign_level_reports;
pub mod get_keyword_level_reports;
pub mod get_search_term_level_reports;
pub mod get_user_acl;

//
//
//
pub(crate) mod endpoint_prelude {
    pub(crate) use apple_web_service_endpoint::{
        http, Body, Endpoint, EndpointParseResponseOutput, Request, Response,
    };
}
