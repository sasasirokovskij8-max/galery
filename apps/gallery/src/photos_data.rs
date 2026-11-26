// Автоматически сгенерировано convert_photos.py
pub const PHOTO_SIZE: usize = 232;

pub struct Photo {
    pub title: &'static str,
    pub data: &'static [u8],
}

pub static PHOTOS: &[Photo] = &[
    Photo { title: "Demo Card", data: include_bytes!("../assets/photo_0.bin") },
];
