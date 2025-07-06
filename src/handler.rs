use std::collections::HashSet;
use std::fs;
use axum::{Json, response::IntoResponse};
use serde_json::{Value, json, to_value};
use crate::presenter::{res_success, res_error, res_error_msg};
use std::path::Path;
use indexmap::IndexMap;
use crate::model::{FieldRequest, SearchRequest};
use crate::utils::load_model_fields;



pub async fn get_data(Json(payload): Json<SearchRequest>) -> impl IntoResponse {
    let json_path = "src/data/reviews.jsonl";

    if !Path::new(json_path).exists() {
        return res_success(Vec::<Value>::new());
    }

    let reviews: Vec<Value> = match fs::read_to_string(json_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => return res_success(Vec::<Value>::new()),
    };

    let model_fields = load_model_fields();
    let search_lower = payload.query.to_lowercase();

    let filtered: Vec<Value> = if search_lower.is_empty() {
        reviews
    } else {
        reviews
            .into_iter()
            .filter(|item| {
                model_fields.iter().any(|field| {
                    item.get(field)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_lowercase().contains(&search_lower))
                        .unwrap_or(false)
                })
            })
            .collect()
    };

    let mapped_reviews: Vec<Value> = filtered
        .into_iter()
        .map(|item| {
            let mut ordered = IndexMap::new();
            for field in &model_fields {
                let value = item.get(field).cloned().unwrap_or_else(|| json!(""));
                ordered.insert(field.clone(), value);
            }
            to_value(ordered).unwrap()
        })
        .collect();

    res_success(mapped_reviews)
}

pub async fn create_data(json: Result<Json<Value>, axum::extract::rejection::JsonRejection>) -> impl IntoResponse {
    let model_fields = load_model_fields();

    let payload = match json {
        Ok(Json(value)) => value,
        Err(err) => return res_error(err),
    };
    let allowed_fields: HashSet<_> = model_fields.iter().cloned().collect();
    let payload_fields: HashSet<_> = payload.as_object().unwrap().keys().cloned().collect();

    for field in &model_fields {
        if !payload.get(field).is_some() {
            return res_error_msg(format!("{} is required", field));
        }
    }

    if !payload_fields.is_subset(&allowed_fields) {
        return res_error_msg("payload contains unexpected fields");
    }
    let json_path = "src/data/reviews.jsonl";

    if let Err(err) = std::fs::create_dir_all("src/data") {
        return res_error(err);
    }

    let mut reviews: Vec<Value> = if Path::new(json_path).exists() {
        match std::fs::read_to_string(json_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
            Err(_) => vec![],
        }
    } else {
        vec![]
    };

    reviews.push(payload.clone());

    let formatted = serde_json::to_string_pretty(&reviews).unwrap();

    if let Err(err) = std::fs::write(json_path, formatted) {
        return res_error(err);
    }
    res_success("create successful")
}


pub async fn get_field() -> impl IntoResponse {
    let fields = load_model_fields();
    res_success(fields)
}

pub async fn create_field(json: Result<Json<FieldRequest>, axum::extract::rejection::JsonRejection>) -> impl IntoResponse {
    let req = match json {
        Ok(Json(data)) => data,
        Err(err) => return res_error_msg(format!("{}", err)),
    };

    let path = "src/data/model.json";

    let mut fields: Vec<String> = if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => vec![],
        }
    } else {
        vec![]
    };

    if fields.contains(&req.field) {
        return res_error_msg(format!("field '{}' already exists", req.field));
    }

    fields.push(req.field.clone());

    match fs::write(path, serde_json::to_string_pretty(&fields).unwrap()) {
        Ok(_) => res_success("created successful"),
        Err(_) => res_error_msg("failed to write to model.json"),
    }
}