pub mod models;
pub mod routes;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use actix_web::{error, http::StatusCode, HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};
use models::ImageError;

const UNITS: &[&str] = &["B", "kB", "MB", "GB", "TB", "PB"];

#[derive(Debug, Display, Error)]
pub enum ImageResponseError {
  #[display(fmt = "image not found")]
  ImageNotFound,

  #[display(fmt = "no multipart fields")]
  NoField,

  #[display(fmt = "route not found")]
  RouteNotFound,

  #[display(fmt = "authorization invalid")]
  NoAuth,
}

impl error::ResponseError for ImageResponseError {
  fn error_response(&self) -> HttpResponse {
    HttpResponseBuilder::new(self.status_code()).json(ImageError {
      code: self.status_code().as_u16(),
      message: self.to_string(),
    })
  }

  fn status_code(&self) -> StatusCode {
    use ImageResponseError::*;

    match *self {
      ImageNotFound | RouteNotFound => StatusCode::NOT_FOUND,
      NoField => StatusCode::BAD_REQUEST,
      NoAuth => StatusCode::FORBIDDEN,
    }
  }
}

pub fn bytes_to_human(mut bytes: f64) -> String {
  let mut num = 0;
  while bytes > 1024_f64 {
    bytes /= 1024_f64;
    num += 1;
  }

  format!("{:.2} {}", bytes, UNITS[num])
}

// export function bytesToRead(bytes: number) {
//   const units = ['B', 'kB', 'MB', 'GB', 'TB', 'PB'];
//   let num = 0;

//   while (bytes > 1024) {
//     bytes /= 1024;
//     ++num;
//   }

//   return `${bytes.toFixed(1)} ${units[num]}`;
// }