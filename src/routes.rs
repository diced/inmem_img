use std::{iter, path::Path, sync::Arc};

use actix_web::{HttpRequest, HttpResponse, Result, get, http, post, web::{self, Bytes}};
use actix_multipart::Multipart;
use futures::StreamExt;
use rand::{Rng, distributions::Alphanumeric, thread_rng};

use crate::{ImageResponseError, models::{Image, SendImage, State}};

#[post("/api/upload")]
pub async fn api_upload_image(
  req: HttpRequest,
  mut payload: Multipart,
  state: web::Data<Arc<State>>,
) -> Result<HttpResponse, ImageResponseError> {
  let auth = match get_authorization(&req) {
    Some(auth) => auth,
    None => return Err(ImageResponseError::NoAuth)
  };

  if auth != state.config.authorization {
    return Err(ImageResponseError::NoAuth)
  }

  let stored_images = &state.stored_images;

  let mut field = match payload.next().await {
    Some(field) => field.unwrap(),
    None => return Err(ImageResponseError::NoField)
  };

  let content_disposition = field
    .content_disposition()
    .unwrap();
  let filename = content_disposition
    .get_filename()
    .unwrap();
  let content_type = field.content_type().to_string();

  let ext = match Path::new(filename).extension() {
    Some(ext) => ext.to_str().unwrap(),
    None => "bin"
  };

  let mut buffer: Vec<u8> = Vec::new();

  while let Some(chunk) = field.next().await {
    let chunk = chunk.unwrap();
    buffer.append(&mut chunk.to_vec())
  }

  let mut rng = thread_rng();
  let id: String = iter::repeat(())
    .map(|()| rng.sample(Alphanumeric))
    .map(char::from)
    .take(state.config.rand_length)
    .collect();
  let name = format!("{}.{}", id, ext);

  stored_images.insert(name.clone(), Image {
    bytes: buffer,
    content_type,
    id: name.clone()
  });

  Ok(HttpResponse::Ok().body(name))
}

#[get("/api/file/{file}")]
pub async fn api_get_image(
  file: web::Path<String>,
  state: web::Data<Arc<State>>,
) -> Result<HttpResponse, ImageResponseError> {
  let stored_images = &state.stored_images;
  
  let image = match stored_images.get(file.as_str()) {
    Some(i) => i,
    None => return Err(ImageResponseError::ImageNotFound)
  };

  Ok(HttpResponse::Ok().json(SendImage::from_image(image.value())))
}

#[get("/{file}")]
pub async fn get_image(
  file: web::Path<String>,
  state: web::Data<Arc<State>>,
) -> Result<HttpResponse, ImageResponseError> {
  let stored_images = &state.stored_images;
  
  let image = match stored_images.get(file.as_str()) {
    Some(i) => i,
    None => return Err(ImageResponseError::ImageNotFound)
  };

  let bytes = Bytes::from(image.bytes.clone());

  Ok(HttpResponse::Ok().content_type(image.content_type.clone()).body(bytes))
}

pub async fn not_found() -> Result<HttpResponse, ImageResponseError> {
  Err(ImageResponseError::RouteNotFound)
}

fn get_authorization<'a>(req: &'a HttpRequest) -> Option<&'a str> {
  req.headers().get(http::header::AUTHORIZATION)?.to_str().ok()
}