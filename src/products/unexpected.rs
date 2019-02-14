use hyper::*;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use mime::APPLICATION_JSON;
use gotham::handler::IntoResponse;
use chrono::Local;
use failure::Error;

use crate::products::expected::*;


#[derive(Debug, Clone, Serialize, Deserialize, Fail)]
/// Unexpected check result
pub enum Unexpected {

    /// Failed to pass domain expectation
    #[fail(display = "Domain expectation: {} has failed for: {}", _1, _0)]
    FailedDomain (String, DomainExpectation),

    /// Failed to pass page expectation
    #[fail(display = "Page expectation: {} has failed for page: {}", _1, _0)]
    FailedPage (String, PageExpectation),

    /// Failed to read/parse JSON
    #[fail(display = "JSON parse failure for: {}", _0)]
    FailedJson (String),

    /// Failed to connect/read from remote
    #[fail(display = "Remote access failure for: {}", _0)]
    FailedRemote (String),

}


#[derive(Debug, Clone, Serialize, Deserialize)]
/// Story holds errornous state
pub struct Story {

    /// Story - timestamp
    timestamp: i64,

    /// Story - failure count
    count: u64,

    /// Story - keep history of unexpected results
    error: Unexpected,

}


impl Story {

    /// New story
    pub fn new(count: u64, error: Unexpected) -> Story {
        Story {
            timestamp: Local::now().timestamp(),
            count,
            error,
        }
    }

}


#[derive(Debug, Clone, Serialize, Deserialize)]
/// History is list of Stories
pub struct History {

    /// Stories list
    pub list: Vec<Story>,

}


impl History {

    /// New History
    pub fn new(first: Story) -> History {
        History {
            list: vec!(first)
        }
    }


    /// Append Story to History
    pub fn append(&self, story: Story) -> History {
        History {
            list: [self.list.clone(), vec!(story)].concat()
        }
    }

}


/// Implement Gotham response for History:
impl IntoResponse for History {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            APPLICATION_JSON,
            serde_json::to_string(&self.list)
                .unwrap_or_else(|_| String::from("{\"status\": \"History serialization failure\"}")),
        )
    }
}
