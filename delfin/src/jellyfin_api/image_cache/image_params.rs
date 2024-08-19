/// Parameters for the Image API endpoint
#[derive(Clone, Debug)]
pub struct ImageParams {
    /// Size constraints for image resizing
    pub size: ImageSize,
    /// Quality setting (0-100), defaults to 96
    pub quality: u16,
}

impl ImageParams {
    pub fn new(size: ImageSize) -> Self {
        Self {
            size,
            // Backend API default quality is 90 but for some reason we use 96 here
            quality: 96,
        }
    }

    pub fn fill_width(size: u16) -> Self {
        Self::new(ImageSize::FillWidth(size))
    }

    pub fn fill_height(size: u16) -> Self {
        Self::new(ImageSize::FillHeight(size))
    }

    pub fn max_width(size: u16) -> Self {
        Self::new(ImageSize::MaxWidth(size))
    }

    pub fn max_height(size: u16) -> Self {
        Self::new(ImageSize::MaxHeight(size))
    }

    pub fn quality(mut self, quality: u16) -> Self {
        self.quality = quality;
        self
    }
}

impl std::fmt::Display for ImageParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}&quality={}", self.size, self.quality)
    }
}

#[derive(Clone, Debug)]
pub enum ImageSize {
    FillWidth(u16),
    FillHeight(u16),
    MaxWidth(u16),
    MaxHeight(u16),
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FillWidth(w) => write!(f, "fillWidth={}", w),
            Self::FillHeight(h) => write!(f, "fillHeight={}", h),
            Self::MaxWidth(w) => write!(f, "maxWidth={}", w),
            Self::MaxHeight(h) => write!(f, "maxHeight={}", h),
        }
    }
}

impl ImageSize {
    pub fn key_val(&self) -> (&'static str, u16) {
        match self {
            Self::FillWidth(w) => ("fillWidth", *w),
            Self::FillHeight(h) => ("fillHeight", *h),
            Self::MaxWidth(w) => ("maxWidth", *w),
            Self::MaxHeight(h) => ("maxHeight", *h),
        }
    }
}
