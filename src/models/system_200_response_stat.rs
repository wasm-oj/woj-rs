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
pub struct System200ResponseStat {
    /// Number of users
    #[serde(rename = "user")]
    pub user: f32,
    /// Number of submissions
    #[serde(rename = "submission")]
    pub submission: f32,
    /// Number of problems
    #[serde(rename = "problem")]
    pub problem: f32,
}

impl System200ResponseStat {
    pub fn new(user: f32, submission: f32, problem: f32) -> System200ResponseStat {
        System200ResponseStat {
            user,
            submission,
            problem,
        }
    }
}


