// src/phi/mod.rs
use ::sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64,
}

impl Rectangle {
  /// Generates an SDL-compatible Rect equiv to `self`
  /// Panics if it could not be created, for example if a
  /// coodinate of a corner overflows an `i32`.
  pub fn to_sdl(self) -> Option<SdlRect> {
    // Reject negative width & height
    assert!(self.w >= 0.0 && self.h >= 0.0);
    // SdlRect::new : `(i32,i32,i32,i32)` -> Result<Option<SdlRect>>
    SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
      .unwrap()
  }

  /// Return sa (perhaps moved) rectangle which is contained by a
  /// `parent` rectangle. If it can indeed by moved to fit, return
  /// `Some(result)` otherwise, `None`
  pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
    // It must be smaller than the parent rectangle to fit in it.
    if self.w > parent.w || self.h > parent.h {
      return None;
    }

    Some(Rectangle {
      w: self.w,
      h: self.h,
      x: if self.x < parent.x { parent.x }
         else if self.x + self.w >= parent.x + parent.w { parent.x }
         else { self.x },
      y: if self.y < parent.y { parent.y }
         else if self.y + self.w >= parent.y + parent.w { parent.y }
         else { self.y },
    })
  }

  pub fn contains(&self, rect: Rectangle) -> bool {
    let xmin = rect.x;
    let xmax = xmin + rect.w;
    let ymin = rect.y;
    let ymax = ymin + rect.h;

    xmin >= self.x && xmin <= (self.x + self.w) &&
    xmax >= self.x && xmax <= (self.x + self.w) &&
    ymin >= self.y && ymin <= (self.y + self.h) &&
    ymax >= self.y && ymax <= (self.y + self.h)
  }

  // pub fn overlaps(&self, other: SdlRect) -> bool {
  //   self.x < (other.x + other.w) as i32 &&
  //   (self.x + self.w) as i32 > other.x &&
  //   self.y < (other.y + other.h) as i32 &&
  //   (self.y + self.h) as i32 > other.y
	// }
}
