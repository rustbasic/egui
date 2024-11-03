use ahash::HashMap;
use egui::{
    load::{BytesPoll, ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint},
    mutex::Mutex,
    ColorImage,
};
use image::ImageFormat;
use std::{mem::size_of, sync::Arc};

type Entry = Result<Arc<ColorImage>, String>;

#[derive(Default)]
pub struct ImageCrateLoader {
    cache: Mutex<HashMap<String, Entry>>,
}

impl ImageCrateLoader {
    pub const ID: &'static str = egui::generate_loader_id!(ImageCrateLoader);
}

fn get_ext_from_image_uri(uri: &str) -> String {
    if uri.trim().is_empty() {
        return String::new();
    }

    let image_uri = uri.to_lowercase();

    let auto_ext: String = if image_uri.contains(".png") {
        "png".to_owned()
    } else if image_uri.contains(".jpg") {
        "jpg".to_owned()
    } else if image_uri.contains(".jpeg") {
        "jpeg".to_owned()
    } else if image_uri.contains(".gif") {
        "gif".to_owned()
    } else if image_uri.contains(".ico") {
        "ico".to_owned()
    } else if image_uri.contains(".svg") {
        "svg".to_owned()
    } else if image_uri.contains(".webp") {
        "webp".to_owned()
    } else if image_uri.contains(".avif") {
        "avif".to_owned()
    } else if image_uri.contains(".bmp") {
        "bmp".to_owned()
    } else if image_uri.contains(".dds") {
        "dds".to_owned()
    } else if image_uri.contains(".exr") {
        "exr".to_owned()
    } else if image_uri.contains(".ff") {
        "ff".to_owned()
    } else if image_uri.contains(".hdr") {
        "hdr".to_owned()
    } else if image_uri.contains(".pbm") {
        "pbm".to_owned()
    } else if image_uri.contains(".pam") {
        "pam".to_owned()
    } else if image_uri.contains(".ppm") {
        "ppm".to_owned()
    } else if image_uri.contains(".pgm") {
        "pgm".to_owned()
    } else if image_uri.contains(".qoi") {
        "qoi".to_owned()
    } else if image_uri.contains(".tif") {
        "tif".to_owned()
    } else if image_uri.contains(".tiff") {
        "tiff".to_owned()
    } else if image_uri.contains(".tga") {
        "tga".to_owned()
    } else {
        // If the image format is unknown, it is assumed to be `png`.
        "png".to_owned()
    };

    return auto_ext;
}

fn is_supported_uri(uri: &str) -> bool {
    let ext = get_ext_from_image_uri(uri);
    if ext.is_empty() {
        // `true` because if there's no extension, assume that we support it
        return true;
    }

    // Uses only the enabled image crate features
    ImageFormat::all()
        .filter(ImageFormat::reading_enabled)
        .flat_map(ImageFormat::extensions_str)
        .any(|format_ext| ext == *format_ext)
}

fn is_unsupported_mime(mime: &str) -> bool {
    // Uses only the enabled image crate features
    !ImageFormat::all()
        .filter(ImageFormat::reading_enabled)
        .map(|fmt| fmt.to_mime_type())
        .any(|format_mime| mime == format_mime)
}

impl ImageLoader for ImageCrateLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, ctx: &egui::Context, uri: &str, _: SizeHint) -> ImageLoadResult {
        // three stages of guessing if we support loading the image:
        // 1. URI extension
        // 2. Mime from `BytesPoll::Ready`
        // 3. image::guess_format

        // (1)
        if !is_supported_uri(uri) {
            return Err(LoadError::NotSupported);
        }

        let mut cache = self.cache.lock();
        if let Some(entry) = cache.get(uri).cloned() {
            match entry {
                Ok(image) => Ok(ImagePoll::Ready { image }),
                Err(err) => Err(LoadError::Loading(err)),
            }
        } else {
            match ctx.try_load_bytes(uri) {
                Ok(BytesPoll::Ready { bytes, mime, .. }) => {
                    // (2 and 3)
                    if mime.as_deref().is_some_and(is_unsupported_mime)
                        || image::guess_format(&bytes).is_err()
                    {
                        return Err(LoadError::NotSupported);
                    }

                    log::trace!("started loading {uri:?}");
                    let result = crate::image::load_image_bytes(&bytes).map(Arc::new);
                    log::trace!("finished loading {uri:?}");
                    cache.insert(uri.into(), result.clone());
                    match result {
                        Ok(image) => Ok(ImagePoll::Ready { image }),
                        Err(err) => Err(LoadError::Loading(err)),
                    }
                }
                Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
                Err(err) => Err(err),
            }
        }
    }

    fn forget(&self, uri: &str) {
        let _ = self.cache.lock().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|result| match result {
                Ok(image) => image.pixels.len() * size_of::<egui::Color32>(),
                Err(err) => err.len(),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_support() {
        assert!(is_supported_uri("https://test.png"));
        assert!(is_supported_uri("test.jpeg"));
        assert!(is_supported_uri("http://test.gif"));
        assert!(is_supported_uri("file://test"));
        assert!(!is_supported_uri("test.svg"));
    }
}
