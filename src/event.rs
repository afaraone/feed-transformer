use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TmEvent {
    pub event_id: Option<String>,
    pub primary_event_url: Option<String>,
    pub event_name: Option<String>,
    pub source: Option<String>,
    pub event_status: Option<String>,
    pub event_start_local_date: Option<String>,
    pub event_start_local_time: Option<String>,
    pub event_start_date_time: Option<String>,
    pub event_end_local_date: Option<String>,
    pub event_end_date_time: Option<String>,
    pub onsale_start_date_time: Option<String>,
    pub classification_segment_id: Option<String>,
    pub classification_genre: Option<String>,
    pub classification_sub_type: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub currency: Option<String>,
    pub venue: Option<Venue>,
    pub attractions: Option<Vec<AttractionWrapper>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub venue_id: Option<String>,
    pub venue_name: Option<String>,
    pub venue_city: Option<String>,
    pub venue_state_code: Option<String>,
    pub venue_country_code: Option<String>,
    pub venue_zip_code: Option<String>,
    pub venue_timezone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttractionWrapper {
    pub attraction: Attraction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attraction {
    pub attraction_id: Option<String>,
    pub attraction_name: Option<String>,
    pub classification_segment: Option<String>,
    pub classification_sub_type: Option<String>,
}

// const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";
