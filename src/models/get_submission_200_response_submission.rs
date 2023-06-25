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
pub struct GetSubmission200ResponseSubmission {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "submitter_id")]
    pub submitter_id: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "code_lang")]
    pub code_lang: String,
    #[serde(rename = "problem_id")]
    pub problem_id: String,
    #[serde(rename = "compiler_version")]
    pub compiler_version: String,
    #[serde(rename = "runner_version")]
    pub runner_version: String,
    #[serde(rename = "score", deserialize_with = "Option::deserialize")]
    pub score: Option<f32>,
    #[serde(rename = "cost", deserialize_with = "Option::deserialize")]
    pub cost: Option<f32>,
    #[serde(rename = "memory", deserialize_with = "Option::deserialize")]
    pub memory: Option<f32>,
    #[serde(rename = "details", deserialize_with = "Option::deserialize")]
    pub details: Option<String>,
}

impl GetSubmission200ResponseSubmission {
    pub fn new(id: String, submitter_id: String, status: Status, code: String, code_lang: String, problem_id: String, compiler_version: String, runner_version: String, score: Option<f32>, cost: Option<f32>, memory: Option<f32>, details: Option<String>) -> GetSubmission200ResponseSubmission {
        GetSubmission200ResponseSubmission {
            id,
            submitter_id,
            status,
            code,
            code_lang,
            problem_id,
            compiler_version,
            runner_version,
            score,
            cost,
            memory,
            details,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "WA")]
    Wa,
    #[serde(rename = "SLE")]
    Sle,
    #[serde(rename = "MLE")]
    Mle,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::Running
    }
}
