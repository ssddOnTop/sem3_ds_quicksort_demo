use std::ops::BitXor;
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::Rng;

fn my_swap<T: Into<usize> + BitXor<Output=T> + Copy + PartialEq>(a: &mut T, b: &mut T) where {
    if *a == *b {
        return;
    }
    *a = *a ^ *b;
    *b = *a ^ *b;
    *a = *a ^ *b;
}

fn partition(vec: &mut Vec<u8>, low: isize, high: isize) -> isize {
    let pivot = *vec.get(high as usize).unwrap();
    let mut i = low - 1;
    for j in low..high {
        if *vec.get(j as usize).unwrap() <= pivot {
            i += 1;
            vec.swap(i as usize, j as usize);
        }
    }
    i += 1;
    vec.swap(high as usize, i as usize);
    return i;
}

fn boring_quick_sort(vec: &mut Vec<u8>, low: isize, high: isize) {
    if low < high {
        let pi = partition(vec, low, high);
        boring_quick_sort(vec,low,pi-1);
        boring_quick_sort(vec,pi+1, high);
    }
}

fn quick_sort(mut pixels: Vec<[u8; 4]>) -> Vec<[u8; 4]> {
    if pixels.len() <= 1 {
        return pixels;
    }

    let pivot_index = pixels.len() / 2;
    let pivot = pixels.remove(pivot_index);

    let mut less = Vec::new();
    let mut greater = Vec::new();

    for pixel in pixels {
        if pixel <= pivot {
            less.push(pixel);
        } else {
            greater.push(pixel);
        }
    }

    let mut sorted_pixels = Vec::new();
    sorted_pixels.extend(quick_sort(less));
    sorted_pixels.push(pivot);
    sorted_pixels.extend(quick_sort(greater));

    sorted_pixels
}

fn sort_all_pixels(img: RgbaImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut new = RgbaImage::new(img.width(), img.height());

    println!("Getting pixels...");
    let mut pixels = Vec::new();
    for y in 0..img.height() {
        pixels.push(Vec::new());
        for x in 0..img.width() {
            pixels[y as usize].push(img.get_pixel(x, y).0);
        }
    }

    println!("Quicksorting pixels...");
    let mut sorted_pixels = Vec::new();
    for y in 0..img.height() {
        sorted_pixels.push(quick_sort(pixels[y as usize].clone()));
    }

    println!("Placing pixels...");
    for y in 0..img.height() {
        for x in 0..img.width() {
            new.put_pixel(x, y, Rgba(sorted_pixels[y as usize][x as usize]));
        }
    }
    new
}

fn generate_unsorted_image(width: u32, height: u32) -> RgbaImage {
    let mut rng = rand::thread_rng();
    let mut unsorted_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let random_color: Rgba<u8> = Rgba([
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                255,
            ]);
            unsorted_image.put_pixel(x, y, random_color);
        }
    }

    unsorted_image
}

fn random_sort_pixels(img: RgbaImage, intensity: u8) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let intensity = if intensity > 100 { 100 } else { intensity };

    let mut new = RgbaImage::new(img.width(), img.height());

    let mut pixels = Vec::new();
    let mut sorted_pixels = Vec::new();

    println!("Getting pixels...");
    // Load all of the pixels into the pixels list
    for y in 0..img.height() {
        pixels.push(Vec::new());
        for x in 0..img.width() {
            pixels[y as usize].push(img.get_pixel(x, y).0);
        }
    }

    let mut rng = rand::thread_rng();

    println!("Randomly sorting pixels...");
    for y in 0..img.height() {
        // Pick different starting points for each line
        if rng.gen_range(0..=100) > intensity {
            sorted_pixels.push(pixels[y as usize].clone()); // Don't sort this line of pixels
        } else {
            let minsort = rng.gen_range(3..(pixels[y as usize].len() - 3));
            let maxsort = rng.gen_range(minsort..pixels[y as usize].len());

            let mut sort = Vec::new();
            for x in minsort..maxsort {
                sort.push(pixels[y as usize][x]);
            }
            sort = quick_sort(sort);
            let mut i = 0;
            for x in minsort..maxsort {
                pixels[y as usize][x] = sort[i];
                i += 1;
            }

            sorted_pixels.push(pixels[y as usize].clone());
        }
    }

    println!("Placing pixels...");
    for y in 0..img.height() {
        for x in 0..img.width() {
            new.put_pixel(x, y, Rgba(sorted_pixels[y as usize][x as usize]));
        }
    }
    new
}


fn sort_pixels_pivot(img: RgbaImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut new = RgbaImage::new(img.width(), img.height());

    let mut pixels = Vec::new();
    let mut sorted_pixels = Vec::new();

    println!("Getting pixels...");
    // Load all of the pixels into the pixels list
    for y in 0..img.height() {
        pixels.push(Vec::new());
        for x in 0..img.width() {
            pixels[y as usize].push(img.get_pixel(x, y).0);
        }
    }

    let mut rng = rand::thread_rng();

    println!("Quicksorting pixels...");

    let minsort = rng.gen_range(3..img.width() - 3); // Get sorting pivot

    for y in 0..img.height() {
        let maxsort = rng.gen_range(minsort..pixels[y as usize].len() as u32); // Pick the end of the sorted area on this pixel line
        let mut sort = Vec::new();

        for x in minsort..maxsort {
            let x = x as usize;
            sort.push(pixels[y as usize][x]);
        }

        sort = quick_sort(sort); // Sort the pixels by brightness

        let mut i = 0;
        for x in minsort..maxsort {
            let x = x as usize;
            pixels[y as usize][x] = sort[i];
            i += 1;
        }

        sorted_pixels.push(pixels[y as usize].clone());
    }

    println!("Placing pixels...");
    for y in 0..img.height() {
        for x in 0..img.width() {
            new.put_pixel(x, y, Rgba(sorted_pixels[y as usize][x as usize]));
        }
    }
    new
}


fn main() {
    // let unsorted_image = generate_unsorted_image(800, 800);
    // unsorted_image.save("assets/unsorted_img.png").unwrap();
    let y = sort_all_pixels(image::open("assets/img_4.jpg").unwrap().to_rgba8());
    // let y = sort_pixels_pivot(image::open("assets/small_object.png").unwrap().to_rgba8());
    // let y = random_sort_pixels(image::open("assets/img_4.jpg").unwrap().to_rgba8(), 100);
    // let y = sort_all_pixels(unsorted_image);
    y.save("assets/small_object_rand_sorted.jpg").unwrap();
    // sort_all_pixels("assets/small_object.png");
   /* let obj = File::open("assets/small_object.png").unwrap();
    let mut br = BufReader::new(obj);
    let mut v = vec![];
    br.read_to_end(&mut v).unwrap();
    let len = v.len();*/

    // quick_sort(&mut v, 0, (len-1) as isize);
    // let mut f = File::create("assets/small_object_sorted.png").unwrap();
    // f.write_all(&v).unwrap();
}
