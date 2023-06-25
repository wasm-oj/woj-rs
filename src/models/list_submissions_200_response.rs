/*
 * WASM OJ Wonderland API
 *
 * You can interact with WASM OJ Wonderland through this API
 *
 * The version of the OpenAPI document: 0.0.4
 * Contact: jacob@csie.cool
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSubmissions200Response {
    #[serde(rename = "submissions")]
    pub submissions: Vec<crate::models::ListSubmissions200ResponseSubmissionsInner>,
}

impl ListSubmissions200Response {
    pub fn new(submissions: Vec<crate::models::ListSubmissions200ResponseSubmissionsInner>) -> ListSubmissions200Response {
        ListSubmissions200Response {
            submissions,
        }
    }
}


