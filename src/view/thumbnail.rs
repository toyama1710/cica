use ::image::DynamicImage;
use ::image::GenericImageView;
use iced::advanced::image::Bytes;
use iced::widget::image;
use iced::widget::image::Handle;
use iced::widget::image::Image;
use std::sync::OnceLock;

use crate::model::ImgFileStatus;

static LOADING_THUMBNAIL: OnceLock<Handle> = OnceLock::new();
static NOT_SUPPORTED_FORMAT_THUMBNAIL: OnceLock<Handle> = OnceLock::new();
static IO_ERROR_THUMBNAIL: OnceLock<Handle> = OnceLock::new();

pub fn thumbnail(stat: &ImgFileStatus) -> Image {
    let handle = match stat {
        ImgFileStatus::Image(img) => &img.thumbnail_handle,
        ImgFileStatus::IOerror(_) => io_error_occured_thumbnail_handle(),
        ImgFileStatus::Loading(_) => loading_thumbnail_handle(),
        ImgFileStatus::NotImage(_) => not_supported_format_thumbnail_handle(),
    };
    image(handle).width(64).height(64)
}

pub fn img_thumbnail_handle(img: &DynamicImage) -> Handle {
    let img = img.thumbnail(64, 64);
    let (w, h) = img.dimensions();
    Handle::from_rgba(w, h, img.to_rgba8().into_raw())
}

fn loading_thumbnail_handle() -> &'static Handle {
    LOADING_THUMBNAIL.get_or_init(|| {
        Handle::from_bytes(Bytes::from_static(include_bytes!(
            "../../assets/loading_image.jpg"
        )))
    })
}

fn not_supported_format_thumbnail_handle() -> &'static Handle {
    NOT_SUPPORTED_FORMAT_THUMBNAIL.get_or_init(|| {
        Handle::from_bytes(Bytes::from_static(include_bytes!(
            "../../assets/not_supported_format.jpg"
        )))
    })
}

fn io_error_occured_thumbnail_handle() -> &'static Handle {
    IO_ERROR_THUMBNAIL.get_or_init(|| {
        Handle::from_bytes(Bytes::from_static(include_bytes!(
            "../../assets/IO_error_occured.jpg"
        )))
    })
}
