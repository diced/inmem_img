use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

pub type StoredImages = DashMap<String, Image>;

pub struct State {
  pub stored_images: StoredImages,
  pub config: Arc<ImageConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageError {
  pub code: u16,
  pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
  pub bytes: Vec<u8>,
  pub content_type: String,
  pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendImage {
  pub content_type: String,
  pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsResponse {
  pub bytes: usize,
  pub images: usize,
}

pub struct ImageConfig {
  pub port: u64,
  pub authorization: String,
  pub rand_length: usize,
}

impl SendImage {
  pub fn from_image(image: &Image) -> Self {
    Self {
      content_type: image.content_type.clone(),
      id: image.id.clone(),
    }
  }
}
