# Photo Gallery для CMF Watch Pro 2 (`.wp2app`)

## Быстрый старт
1. Добавьте от 1 до 8 фотографий (`.jpg`, `.png`, `.webp`) в папку `apps/gallery/raw_photos/`.
2. Перейдите в `apps/gallery/` и запустите конвертер:
   ```bash
   pip install pillow
   python convert_photos.py
   ```
   *(В архив уже добавлена тестовая карточка `assets/photo_0.bin` и `src/photos_data.rs`, поэтому проект скомпилируется сразу даже без запуска скрипта.)*
3. Поместите папку `apps/gallery/` в корень вашего форка репозитория `fmctools/apps`.
4. Откройте Pull Request в вашем форке или запустите `make build APP=gallery` локально, чтобы получить готовый `gallery.wp2app`.
5. Установите `gallery.wp2app` на CMF Watch Pro 2 через `fmc.tools` (вкладка **Apps**).
