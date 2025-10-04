use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use std::fs;

pub async fn handle_get_sounds() -> impl IntoResponse {
    match fs::read_dir("resources") {
        Ok(entries) => {
            let sound_files: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_file() {
                            path.file_name()
                                .and_then(|name| name.to_str())
                                .map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            (StatusCode::OK, Json(json!({"sounds": sound_files})))
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Failed to read resources directory: {}", err)
            })),
        ),
    }
}
