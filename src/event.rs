use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TmEvent {
    pub event_id: String,
    pub event_name: String,
    pub classification_segment_id: Option<String>,
}

const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";

impl TmEvent {
    pub fn is_music(&self) -> bool {
        if let Some(classification_segment_id) = &self.classification_segment_id {
            classification_segment_id == TM_MUSIC_CLASSIFICATION_ID
        } else {
            false
        }
    }
}
