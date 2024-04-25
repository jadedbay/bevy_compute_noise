use bevy::render::{render_resource::{ImageDataLayout, TextureFormat}, texture::TextureFormatPixelInfo};

// https://github.com/Commander-lol/bevy_capture_media/blob/trunk/src/render.rs
const COPY_BYTES_PER_ROW_ALIGNMENT: u32 = 256;
pub fn align_byte_size(value: u32) -> u32 {
	value + (COPY_BYTES_PER_ROW_ALIGNMENT - (value % COPY_BYTES_PER_ROW_ALIGNMENT))
}

pub fn get_aligned_size(width: u32, height: u32, pixel_size: u32) -> u32 {
	height * align_byte_size(width * pixel_size)
}

pub fn layout_data(width: u32, height: u32, depth: u32, format: TextureFormat) -> ImageDataLayout {
	ImageDataLayout {
		bytes_per_row: if height > 1 {
			Some(get_aligned_size(width, 1, format.pixel_size() as u32))
		} else {
			None
		},
		// not sure if this is right
		rows_per_image: if depth > 1 {
			Some(depth)
		} else {
			None
		},
		..Default::default()
	}
}