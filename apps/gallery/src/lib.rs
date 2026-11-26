#![no_std]

mod photos_data;

use cmf_wp2_app::{Api, Event};
use cmf_wp2_ui::{Canvas, Color};
use photos_data::{PHOTOS, PHOTO_SIZE};

const SCREEN_W: i32 = 466;
const SCREEN_H: i32 = 466;
const SCALE: i32 = 2; // 232 * 2 = 464 px

struct GalleryState {
    index: usize,
    show_hud: bool,
}

impl GalleryState {
    const fn new() -> Self {
        Self {
            index: 0,
            show_hud: true,
        }
    }

    fn next(&mut self) {
        if !PHOTOS.is_empty() {
            self.index = (self.index + 1) % PHOTOS.len();
        }
    }

    fn prev(&mut self) {
        if !PHOTOS.is_empty() {
            self.index = if self.index == 0 {
                PHOTOS.len() - 1
            } else {
                self.index - 1
            };
        }
    }

    fn toggle_hud(&mut self) {
        self.show_hud = !self.show_hud;
    }
}

fn draw_gallery(canvas: &mut Canvas, state: &GalleryState) {
    canvas.clear(Color::BLACK);

    if PHOTOS.is_empty() {
        canvas.text(130, 220, "NO PHOTOS", Color::WHITE);
        canvas.show();
        return;
    }

    let photo = &PHOTOS[state.index];
    let draw_size = (PHOTO_SIZE as i32) * SCALE;
    let offset_x = (SCREEN_W - draw_size) / 2;
    let offset_y = (SCREEN_H - draw_size) / 2;

    for y in 0..PHOTO_SIZE {
        let row_offset = y * PHOTO_SIZE * 2;
        let py = offset_y + (y as i32) * SCALE;

        for x in 0..PHOTO_SIZE {
            let idx = row_offset + x * 2;
            if idx + 1 >= photo.data.len() {
                break;
            }
            let raw = u16::from_le_bytes([photo.data[idx], photo.data[idx + 1]]);
            let color = Color::from_rgb565(raw);
            let px = offset_x + (x as i32) * SCALE;

            canvas.rect(px, py, SCALE, SCALE, color);
        }
    }

    if state.show_hud {
        canvas.rect(113, 28, 240, 34, Color::from_rgb565(0x18E3));
        canvas.text(145, 37, photo.title, Color::WHITE);

        let count = PHOTOS.len() as i32;
        let dot_gap = 16;
        let total_w = (count - 1) * dot_gap;
        let start_x = (SCREEN_W - total_w) / 2;

        for i in 0..count {
            let dot_color = if i as usize == state.index {
                Color::from_rgb565(0xFA60)
            } else {
                Color::from_rgb565(0x7BEF)
            };
            canvas.rect(start_x + i * dot_gap - 4, 416, 8, 8, dot_color);
        }
    }

    canvas.show();
}

#[no_mangle]
pub extern "C" fn main(api: &Api) -> i32 {
    let mut canvas = Canvas::new(api);
    let mut state = GalleryState::new();

    draw_gallery(&mut canvas, &state);

    loop {
        match api.poll_event() {
            Some(Event::Crown(delta)) => {
                if delta > 0 {
                    state.next();
                } else if delta < 0 {
                    state.prev();
                }
                draw_gallery(&mut canvas, &state);
            }
            Some(Event::TouchUp { x, .. }) => {
                if x < 140 {
                    state.prev();
                } else if x > 326 {
                    state.next();
                } else {
                    state.toggle_hud();
                }
                draw_gallery(&mut canvas, &state);
            }
            Some(Event::Button) => break,
            Some(Event::Resume) => draw_gallery(&mut canvas, &state),
            None => api.sleep_ms(16),
            _ => {}
        }
    }

    0
}
