use std::time::Instant;
use libvips::{ops, VipsApp};
use sha1::{Sha1, Digest};
use std::fs;
use std::io::{Read, Error};

fn generate_thumbnails(image_names: &[&str]) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let thumbnails: Vec<Vec<u8>> = Vec::new();
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    app.concurrency_set(2);

    for image_name in image_names {
        let image_path = &format!("../data/{}", image_name);
        let start_time = Instant::now();

        let resized = ops::thumbnail(image_path, 320).ok().unwrap();
        //optional parameters
        let options = ops::JpegsaveOptions {
            q: 90,
            background: vec![255.0],
            strip: true,
            optimize_coding: true,
            optimize_scans: true,
            interlace: true,
            ..ops::JpegsaveOptions::default()
        };
    
        // alternatively you can use `jpegsave` that will use the default options
        match ops::jpegsave_with_opts(&resized, &format!("output/{}", &image_name),  &options) {
            Err(_) => println!("error: {}", app.error_buffer().unwrap()),
            Ok(_) => println!("Great Success!")
        }

        let elapsed_time = start_time.elapsed().as_millis();

        println!("{} thumbnail took {:.2}ms", image_path, elapsed_time);
        let start_time2 = Instant::now();

        calculate_sha1(image_path);
    
        let elapsed_time2 = start_time2.elapsed().as_millis();

        println!("{} sha1 took {:.2}ms", image_path, elapsed_time2);
    }

    Ok(thumbnails)
}

fn calculate_sha1(filename: &str) -> Result<String, Error> {
    // Open the file
    let mut file = fs::File::open(filename)?;

    // Create a SHA-1 hasher
    let mut hasher = Sha1::new();

    // Read the file in chunks and update the hasher
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash and convert it to a hexadecimal string
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    Ok(hash_hex)
}

fn main() {
    let image_names = [
        "1.jpg",
        "2.jpg",
        "3.jpg",
        "4.jpg",
        "5.jpg",
    ];

    if let Err(err) = generate_thumbnails(&image_names) {
        eprintln!("Error generating thumbnails: {}", err);
    } else {
        println!("Thumbnails generated successfully!");
    }
}
