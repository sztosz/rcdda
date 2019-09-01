use sdl2::render::TextureValueError;

#[derive(Debug)]
pub enum GameError {
  String,
  TextureValueError,
  GenericError(String),
}

impl From<String> for GameError {
  fn from(msg: String) -> GameError {
    GameError::GenericError(msg)
  }
}
impl From<TextureValueError> for GameError {
  fn from(e: TextureValueError) -> GameError {
    let msg = match e {
      TextureValueError::WidthOverflows(i) => format!("WidthOverflows {}", i),
      TextureValueError::HeightOverflows(i) => format!("HeightOverflows {}", i),
      TextureValueError::WidthMustBeMultipleOfTwoForFormat(i, pixel_format_enum) => format!(
        "WidthMustBeMultipleOfTwoForFormat {}, {}",
        i, pixel_format_enum as i32
      ),
      TextureValueError::SdlError(string) => format!("SdlError {}", string),
    };
    GameError::GenericError(msg)
  }
}
