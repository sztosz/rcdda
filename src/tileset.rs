use sdl2::{image::LoadSurface, surface::Surface};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{collections::HashMap, fmt, fs::File, io::BufReader, path::Path};

const TILE_DIR: &'static str = "assets/tiles";
const SPRITE_SHEET_JSON_CONFIG: &'static str = "tile_config.json";

type TileHash = HashMap<String, Tile>;

#[derive(Debug)]
pub struct Tile {
  fg: Option<i32>,
  bg: Option<i32>,
}

pub struct Tileset<'a> {
  // height: u32,
  // width: u32,
  pub tile_height: u32,
  pub tile_width: u32,
  pub tiles: HashMap<String, Tile>,
  // file_path: String,
  pub sprites_surface: Surface<'a>,
}

pub fn load_tileset<'a>() -> Tileset<'a> {
  let (tile_height, tile_width, file_path, tiles) = parse_tile_config();
  let sprites_surface: Surface = Surface::from_file(Path::new(TILE_DIR).join(&file_path)).unwrap();
  let tileset = Tileset {
    tile_height,
    tile_width,
    tiles: tiles,
    sprites_surface: sprites_surface,
  };
  tileset
}

fn parse_tile_config() -> (u32, u32, String, TileHash) {
  let path = Path::new(TILE_DIR).join(SPRITE_SHEET_JSON_CONFIG);
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  let json: Value = serde_json::from_reader(reader).unwrap();
  let (tile_height, tile_width) = parse_tile_info(&json);
  let (file_path, tiles) = parse_tiles_new(&json);
  (tile_height, tile_width, file_path, tiles)
}

fn parse_tile_info(json: &Value) -> (u32, u32) {
  let height = json["tile_info"][0]["height"].as_i64().unwrap() as u32;
  let width = json["tile_info"][0]["width"].as_i64().unwrap() as u32;
  (height, width)
}

fn parse_tiles_new(json: &Value) -> (String, TileHash) {
  let tiles_new = json["tiles-new"][0]["tiles"].as_array().unwrap();
  let file_name = json["tiles-new"][0]["file"].as_str().unwrap().to_string();
  let mut tiles: TileHash = HashMap::new();
  println!("{:?}", tiles_new.len());
  for tile_new in tiles_new {
    if tile_new["id"].is_array() {
      for id in tile_new["id"].as_array().unwrap() {
        insert_tile(&id, &tile_new["fg"], &tile_new["bg"], &mut tiles);
      }
    } else {
      insert_tile(&tile_new["id"], &tile_new["fg"], &tile_new["bg"], &mut tiles);
    }
  }
  (file_name, tiles)
}

fn insert_tile(id: &Value, fg: &Value, bg: &Value, tiles: &mut TileHash) {
  tiles.insert(
    id.as_str().unwrap().to_string(),
    Tile {
      fg: match fg.as_i64() {
        Some(x) => Some(x as i32),
        None => None,
      },
      bg: match bg.as_i64() {
        Some(x) => Some(x as i32),
        None => None,
      },
    },
  );
}

// sprite_surface.set_color_key(true, TRANSPARENT).unwrap();

// let texture_creator = canvas.texture_creator();
// let mut spritesheet: Texture = texture_creator.create_texture_from_surface(&sprite_surface)?;
// spritesheet.set_blend_mode(BlendMode::Add);

// let tex_query: TextureQuery = spritesheet.query();

// println!("tex_query.width == {}", tex_query.width);
// println!(" SPRITE_W * SPRITE_COLS == {}", SPRITE_W * SPRITE_COLS);
// println!("tex_query.height == {}", tex_query.height);
// println!("SPRITE_H * SPRITE_ROWS == {}", SPRITE_H * SPRITE_ROWS);
// assert!(tex_query.width == SPRITE_W * SPRITE_COLS);
// assert!(tex_query.height == SPRITE_H * SPRITE_ROWS);

//  let raw_sprite: Surface = raw_sprite(&sprite_surface, src_rect).unwrap();
//     let sprite_tex: Texture = texture_creator
//       .create_texture_from_surface(&raw_sprite)
//       .unwrap();

// fn raw_sprite<'a>(spritesheet: &Surface, sprite_rect: Rect) -> Result<Surface<'a>, String> {
//   let mut tile: Surface = create_tile(sprite_rect)?;
//   spritesheet.blit(sprite_rect, &mut tile, None)?;
//   Ok(tile)
// }

// fn create_tile<'a>(size_rect: Rect) -> Result<Surface<'a>, String> {
//   let tile: Surface = Surface::new(
//     size_rect.width(),
//     size_rect.height(),
//     PixelFormatEnum::RGB888,
//   )?;
//   Ok(tile)
// }
