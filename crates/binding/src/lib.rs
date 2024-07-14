use minigrep;
use napi_derive::napi;

#[napi(object)]
pub struct Params {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

#[napi]
pub fn search(params: Params) -> () {
    let config = minigrep::Config {
        query: params.query,
        filename: params.filename,
        ignore_case: params.ignore_case,
    };

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
    }
}
