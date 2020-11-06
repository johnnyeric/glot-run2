use crate::glot_run::config;
use crate::glot_run::api;
use crate::glot_run::language;
use crate::glot_run::datastore;
use crate::glot_run::file;


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, serde::Serialize)]
pub struct Language {
    name: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<api::SuccessResponse, api::ErrorResponse> {

    let data_root = config.server.data_root.lock().unwrap();
    let mut languages = datastore::list_values::<language::Language>(&data_root.languages_path()).map(|languages| {
        languages
            .iter()
            .map(to_language)
            .collect::<Vec<Language>>()
    }).map_err(handle_datastore_error)?;

    languages.sort();
    languages.dedup();

    api::prepare_json_response(&languages)
}


fn to_language(language: &language::Language) -> Language {
    Language{
        name: language.name.clone(),
    }
}

fn handle_datastore_error(err: file::ReadJsonError) -> api::ErrorResponse{
    api::ErrorResponse{
        status_code: 500,
        body: api::ErrorBody{
            error: "datastore".to_string(),
            message: err.to_string(),
        }
    }
}
