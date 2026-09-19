use reqwest::header::HeaderMap;

use crate::SourceConfiguration;

impl From<HeaderMap> for SourceConfiguration {
    fn from(value: HeaderMap) -> Self {
        let headers = value
            .iter()
            .filter_map(|(k, v)| {
                v.to_str()
                    .ok()
                    .map(|val| (k.as_str().to_string(), val.to_string()))
            })
            .collect();

        Self { headers }
    }
}
