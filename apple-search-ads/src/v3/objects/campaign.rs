// https://developer.apple.com/documentation/apple_search_ads/campaign

use std::str::FromStr as _;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use strum::EnumString;

use crate::v3::{
    objects::{
        campaign_country_or_region_serving_state_reasons::CampaignCountryOrRegionServingStateReasons,
        loc_invoice_details::LOCInvoiceDetails, money::Money,
    },
    types::{payment_model::PaymentModel, region::Region},
};

#[derive(Deserialize, Debug, Clone)]
pub struct Campaign {
    #[serde(rename = "adChannelType")]
    pub ad_channel_type: CampaignAdChannelType,

    #[serde(rename = "adamId")]
    pub adam_id: u64,

    #[serde(rename = "budgetAmount")]
    pub budget_amount: Money,

    #[serde(rename = "budgetOrders")]
    pub budget_orders: Vec<u64>,

    #[serde(rename = "countriesOrRegions")]
    pub countries_or_regions: Vec<Region>,

    #[serde(rename = "countryOrRegionServingStateReasons")]
    pub country_or_region_serving_state_reasons: CampaignCountryOrRegionServingStateReasons,

    #[serde(rename = "dailyBudgetAmount")]
    pub daily_budget_amount: Option<Money>,

    pub deleted: bool,

    #[serde(rename = "displayStatus")]
    pub display_status: CampaignDisplayStatus,

    #[serde(default)]
    #[serde(with = "campaign_option_date_format")]
    #[serde(rename = "endTime")]
    pub end_time: Option<DateTime<Utc>>,

    pub id: u64,

    #[serde(rename = "locInvoiceDetails")]
    pub loc_invoice_details: LOCInvoiceDetails,

    #[serde(with = "campaign_date_format")]
    #[serde(rename = "modificationTime")]
    pub modification_time: DateTime<Utc>,

    pub name: String,

    #[serde(rename = "orgId")]
    pub org_id: u64,

    #[serde(rename = "paymentModel")]
    pub payment_model: PaymentModel,

    #[serde(rename = "servingStateReasons")]
    pub serving_state_reasons: Option<Vec<CampaignServingStateReason>>,

    #[serde(rename = "servingStatus")]
    pub serving_status: CampaignServingStatus,

    pub status: CampaignStatus,

    #[serde(with = "campaign_date_format")]
    #[serde(rename = "startTime")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "supplySources")]
    pub supply_sources: Vec<CampaignSupplySource>,
}

//
pub mod campaign_option_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            Utc.datetime_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }
}

pub mod campaign_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

//
#[derive(EnumString, PartialEq, Debug, Clone)]
pub enum CampaignAdChannelType {
    #[allow(clippy::upper_case_acronyms)]
    SEARCH,
    #[allow(clippy::upper_case_acronyms)]
    DISPLAY,
    #[strum(disabled)]
    Other(String),
}
impl<'de> Deserialize<'de> for CampaignAdChannelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        Ok(Self::from_str(str.as_ref()).unwrap_or_else(|_| Self::Other(str)))
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum CampaignDisplayStatus {
    #[allow(clippy::upper_case_acronyms)]
    RUNNING,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    ON_HOLD,
    #[allow(clippy::upper_case_acronyms)]
    PAUSED,
    #[allow(clippy::upper_case_acronyms)]
    DELETED,
}

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum CampaignServingStateReason {
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    NO_PAYMENT_METHOD_ON_FILE,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    MISSING_BO_OR_INVOICING_FIELDS,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    PAUSED_BY_USER,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    DELETED_BY_USER,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    CAMPAIGN_END_DATE_REACHED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    CAMPAIGN_START_DATE_IN_FUTURE,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    DAILY_CAP_EXHAUSTED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    TOTAL_BUDGET_EXHAUSTED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    CREDIT_CARD_DECLINED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_ELIGIBLE,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_ELIGIBLE_SEARCHADS,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APP_NOT_PUBLISHED_YET,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    BO_START_DATE_IN_FUTURE,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    BO_END_DATE_REACHED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    BO_EXHAUSTED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    ORG_PAYMENT_TYPE_CHANGED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    ORG_SUSPENDED_POLICY_VIOLATION,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    ORG_SUSPENDED_FRAUD,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    ORG_CHARGE_BACK_DISPUTED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    PAUSED_BY_SYSTEM,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    LOC_EXHAUSTED,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    TAX_VERIFICATION_PENDING,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_AGENT_UNKNOWN,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_FRENCH_BIZ_UNKNOWN,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    SAPIN_LAW_FRENCH_BIZ,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    NO_ELIGIBLE_COUNTRIES,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    AD_GROUP_MISSING,
}

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum CampaignServingStatus {
    #[allow(clippy::upper_case_acronyms)]
    RUNNING,
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    NOT_RUNNING,
}

#[derive(Deserialize, PartialEq, strum::Display, Debug, Clone)]
pub enum CampaignStatus {
    #[allow(clippy::upper_case_acronyms)]
    ENABLED,
    #[allow(clippy::upper_case_acronyms)]
    PAUSED,
}

#[derive(EnumString, PartialEq, Debug, Clone)]
pub enum CampaignSupplySource {
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    APPSTORE_SEARCH_RESULTS,
    #[allow(clippy::upper_case_acronyms)]
    NEWS,
    #[allow(clippy::upper_case_acronyms)]
    STOCKS,
    #[strum(disabled)]
    Other(String),
}
impl<'de> Deserialize<'de> for CampaignSupplySource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        Ok(Self::from_str(str.as_ref()).unwrap_or_else(|_| Self::Other(str)))
    }
}
