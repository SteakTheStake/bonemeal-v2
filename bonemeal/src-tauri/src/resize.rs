use std::path::Path;
use glob::glob;
use image::{ImageFormat, GenericImageView, DynamicImage, ImageBuffer, Rgba};
use image::imageops::FilterType;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
#[tauri::command]
pub fn resize_images(folder_path: String, max_width: u32, dither: bool, progress: tauri::State<ProgressState>) -> Result<(), String> {
    let total_files = count_png_files(&folder_path)?;
    let progress_state = progress.0.clone();

    let mut processed_files = 0;

    let glob_pattern = format!("{}/**/*.png", folder_path);
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with("_n.png") || file_name.ends_with("_s.png") {
                    resize_image(&path, max_width, FilterType::Triangle, dither)?;
                } else {
                    resize_image(&path, max_width, FilterType::Nearest, dither)?;
                }
                processed_files += 1;
                progress_state.store(processed_files, Ordering::SeqCst);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn count_png_files(folder_path: &str) -> Result<usize, String> {
    let glob_pattern = format!("{}/**/*.png", folder_path);
    let entries = glob(&glob_pattern).expect("Failed to read glob pattern");
    Ok(entries.count())
}

pub struct ProgressState(pub Arc<AtomicUsize>);
fn resize_image(path: &Path, max_width: u32, filter: FilterType, dither: bool) -> Result<(), String> {
    let img = image::open(path).map_err(|e| e.to_string())?;
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;

    let new_width = max_width;
    let new_height = (new_width as f32 / aspect_ratio) as u32;

    let resized_img = img.resize(new_width, new_height, filter);

    let output_img = if dither {
        let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = resized_img.to_rgba8();
        let mut gray_buffer = image::imageops::grayscale(&img_buffer);
        image::imageops::dither(&mut gray_buffer, &image::imageops::colorops::BiLevel);
        DynamicImage::ImageLuma8(gray_buffer)
    } else {
        resized_img
    };

    let output_path = path.to_owned();
    output_img.save_with_format(output_path, ImageFormat::Png).map_err(|e| e.to_string())?;

    Ok(())
}