use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpErrorOut {
    pub code: String,
    pub detail: String,
}

impl HttpErrorOut {
    pub fn new(code: String, detail: String) -> HttpErrorOut {
        HttpErrorOut { code, detail }
    }
}
