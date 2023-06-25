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
pub struct ListProblems200Response {
    /// List of problems
    #[serde(rename = "problems")]
    pub problems: Vec<crate::models::ListProblems200ResponseProblemsInner>,
}

impl ListProblems200Response {
    pub fn new(problems: Vec<crate::models::ListProblems200ResponseProblemsInner>) -> ListProblems200Response {
        ListProblems200Response {
            problems,
        }
    }
}


