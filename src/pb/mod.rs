use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let data = decode_config(s, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

impl filter::FilterType {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::FilterType::Unspecified => None,
            filter::FilterType::Oceanic => Some("oceanic"),
            filter::FilterType::Islands => Some("islands"),
            filter::FilterType::Marine => Some("marine"),
        }
    }
}

impl From<resize::FilterType> for SamplingFilter {
    fn from(v: resize::FilterType) -> Self {
        match v {
            resize::FilterType::Undefined => SamplingFilter::Nearest,
            resize::FilterType::Nearest => SamplingFilter::Nearest,
            resize::FilterType::Triangle => SamplingFilter::Triangle,
            resize::FilterType::CatmullRom => SamplingFilter::CatmullRom,
            resize::FilterType::Gaussian => SamplingFilter::Gaussian,
            resize::FilterType::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::FilterType::Undefined as i32,
            })),
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: resize::FilterType) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }
    pub fn new_filter(filter: filter::FilterType) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }
    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::FilterType::CatmullRom);
        let spec2 = Spec::new_filter(filter::FilterType::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap())
    }
}
