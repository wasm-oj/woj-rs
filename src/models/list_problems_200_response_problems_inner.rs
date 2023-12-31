/*
 * WASM OJ Wonderland API
 *
 * You can interact with WASM OJ Wonderland through this API
 *
 * The version of the OpenAPI document: 0.0.4
 * Contact: jacob@csie.cool
 * Generated by: https://openapi-generator.tech
 */

/// ListProblems200ResponseProblemsInner : Problem



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListProblems200ResponseProblemsInner {
    /// Submission count of each status
    #[serde(rename = "sub")]
    pub sub: ::std::collections::HashMap<String, f32>,
    /// Problem ID
    #[serde(rename = "id")]
    pub id: String,
    /// Problem name
    #[serde(rename = "name")]
    pub name: String,
    /// Problem tags
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl ListProblems200ResponseProblemsInner {
    /// Problem
    pub fn new(sub: ::std::collections::HashMap<String, f32>, id: String, name: String, tags: Vec<String>) -> ListProblems200ResponseProblemsInner {
        ListProblems200ResponseProblemsInner {
            sub,
            id,
            name,
            tags,
        }
    }
}


