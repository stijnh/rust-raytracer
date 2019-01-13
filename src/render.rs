use crate::integrator::WhittedIntegrator as Integrator;
use crate::scene::Scene;
use crate::texture::Color;
use arrayvec::ArrayVec;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Mutex;
use std::time::SystemTime;

pub fn parallel_render(scene: &Scene, integrator: &Integrator) -> Box<[Color]> {
    let (width, height) = scene.camera.dimensions();
    let mut buffer = vec![];

    let style = ProgressStyle::default_bar()
        .template("  {bar:50} {percent}%, {elapsed_precise} (eta: {eta_precise})")
        .progress_chars("\u{2588}\u{2592}\u{2591}");
    let progress = ProgressBar::new((width * height) as u64);
    progress.set_style(style);
    let progress_ref = &progress;

    let before = SystemTime::now();
    (0..width * height)
        .into_par_iter()
        .with_min_len(100)
        .with_max_len(100)
        .map(move |index| {
            let x = index % width;
            let y = index / width;

            if x == 0 {
                progress_ref.inc(width as u64);
            }

            integrator.calculate_pixel(scene, x, y)
        })
        .collect_into_vec(&mut buffer);
    let elapsed = before.elapsed().unwrap();
    let time = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64) * 1e-9;

    let minutes = (time / 60.0).floor() as i32;
    let seconds = (time % 60.0).ceil() as i32;

    progress.finish_and_clear();
    println!(
        "Rendered {}x{}={} pixels in {:02}:{:02} ({:.3} sec/pixel)",
        width,
        height,
        width * height,
        minutes,
        seconds,
        time / (width as f64 * height as f64)
    );

    buffer.into_boxed_slice()
}

pub fn parallel_render_image(scene: &Scene, integrator: &Integrator) -> RgbImage {
    let (width, height) = scene.camera.dimensions();
    let buffer = parallel_render(scene, integrator);

    let pixels = buffer
        .iter()
        .flat_map(|c| -> ArrayVec<_> {
            [
                (c[0] * 256.0).floor().max(0.0).min(255.0) as u8,
                (c[1] * 256.0).floor().max(0.0).min(255.0) as u8,
                (c[2] * 256.0).floor().max(0.0).min(255.0) as u8,
            ]
            .into()
        })
        .collect::<Vec<_>>();

    RgbImage::from_raw(width as u32, height as u32, pixels).expect("dimensions mismatch")
}
