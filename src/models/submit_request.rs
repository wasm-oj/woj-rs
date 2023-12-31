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
pub struct SubmitRequest {
    /// Problem ID
    #[serde(rename = "problem")]
    pub problem: String,
    /// Source code
    #[serde(rename = "code")]
    pub code: String,
    /// Programming language of the source code
    #[serde(rename = "lang")]
    pub lang: Lang,
}

impl SubmitRequest {
    pub fn new(problem: String, code: String, lang: Lang) -> SubmitRequest {
        SubmitRequest {
            problem,
            code,
            lang,
        }
    }
}

/// Programming language of the source code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Lang {
    #[serde(rename = "rs")]
    Rs,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
}

impl Default for Lang {
    fn default() -> Lang {
        Self::Rs
    }
}

