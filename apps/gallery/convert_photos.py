import os
import struct
from PIL import Image

RAW_DIR = "raw_photos"
ASSETS_DIR = "assets"
RS_OUT = os.path.join("src", "photos_data.rs")
SIZE = 232  # 232x232 -> масштабируется 2x в 464x464 на экране 466x466


def rgb_to_rgb565(r: int, g: int, b: int) -> int:
    return ((r & 0xF8) << 8) | ((g & 0xFC) << 3) | (b >> 3)


def center_crop_square(img: Image.Image) -> Image.Image:
    w, h = img.size
    side = min(w, h)
    left = (w - side) // 2
    top = (h - side) // 2
    return img.crop((left, top, left + side, top + side))


def main():
    os.makedirs(RAW_DIR, exist_ok=True)
    os.makedirs(ASSETS_DIR, exist_ok=True)
    os.makedirs("src", exist_ok=True)

    exts = (".jpg", ".jpeg", ".png", ".webp", ".bmp")
    files = sorted(f for f in os.listdir(RAW_DIR) if f.lower().endswith(exts))

    entries = []

    if not files:
        bin_name = "photo_0.bin"
        bin_path = os.path.join(ASSETS_DIR, bin_name)
        buf = bytearray()
        for y in range(SIZE):
            for x in range(SIZE):
                r = int((x / SIZE) * 255)
                g = int((y / SIZE) * 140)
                b = 220 - int((x / SIZE) * 180)
                buf += struct.pack("<H", rgb_to_rgb565(r, g, b))
        with open(bin_path, "wb") as f:
            f.write(buf)
        entries.append(("Demo Card", bin_name))
    else:
        for idx, fname in enumerate(files[:8]):
            src_path = os.path.join(RAW_DIR, fname)
            img = Image.open(src_path).convert("RGB")
            img = center_crop_square(img).resize((SIZE, SIZE), Image.Resampling.LANCZOS)

            bin_name = f"photo_{idx}.bin"
            bin_path = os.path.join(ASSETS_DIR, bin_name)

            buf = bytearray()
            for r, g, b in img.getdata():
                buf += struct.pack("<H", rgb_to_rgb565(r, g, b))

            with open(bin_path, "wb") as f:
                f.write(buf)

            title = os.path.splitext(fname)[0][:18].replace('"', "")
            entries.append((title, bin_name))

    with open(RS_OUT, "w", encoding="utf-8") as rs:
        rs.write("// Автоматически сгенерировано convert_photos.py\n")
        rs.write(f"pub const PHOTO_SIZE: usize = {SIZE};\n\n")
        rs.write("pub struct Photo {\n")
        rs.write("    pub title: &'static str,\n")
        rs.write("    pub data: &'static [u8],\n")
        rs.write("}\n\n")
        rs.write("pub static PHOTOS: &[Photo] = &[\n")
        for title, bin_name in entries:
            rs.write(
                f'    Photo {{ title: "{title}", data: include_bytes!("../assets/{bin_name}") }},\n'
            )
        rs.write("];\n")

    print(f"Готово: обработано {len(entries)} фото -> {RS_OUT}")


if __name__ == "__main__":
    main()
