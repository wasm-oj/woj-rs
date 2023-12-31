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
pub struct Login400Response {
    #[serde(rename = "message")]
    pub message: String,
}

impl Login400Response {
    pub fn new(message: String) -> Login400Response {
        Login400Response {
            message,
        }
    }
}


