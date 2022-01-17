use image::{self, GenericImageView, Rgb};
use wgpu::Extent3d;

pub fn load_tree_texture(
    device: &wgpu::Device,
) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>, wgpu::Texture, (u32,u32), Extent3d) {
    let diffuse_bytes = include_bytes!("happy-tree.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgb8();

    let dimensions = diffuse_image.dimensions();

    let texture_size = Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let d = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("image texture"),
    });
    (diffuse_rgba, d, dimensions, texture_size)
}
