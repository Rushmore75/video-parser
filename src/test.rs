use std::io::*;
use ffmpeg::format::Pixel;
use rand::Rng;


pub fn small_matrix() {
    let mut matrix = [0u8; 432]; // 144 * 3 = 432
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..50);
    let g = rng.gen_range(0..50);
    let b = rng.gen_range(0..50);

    matrix.iter_mut().enumerate().for_each(|f| {
        let value = match f.0 % 3 {
            0 => r,
            1 => g,
            _ => b,
        };

        *f.1 = value;
    });

    print_raw(&matrix);
}

#[inline]
/// Asuumes a square ig
pub fn print_raw(buf: &[u8]) {
    let mut stdout = stdout();
    let mut lock = stdout.lock();
    lock.write(buf).unwrap();
    stdout.flush().unwrap();
}

/// Must use square images
// pub fn print_square(buf: &[u8], width_hight: usize, line) {
pub fn print_square(frame: &ffmpeg::frame::Video) {
    if frame.format() != Pixel::RGB24 { panic!("Must use Pixel::RGB24"); }
    if frame.width() != frame.height() { panic!("Must be square frames"); } 

    let buf = frame.data(0); 
    let size = frame.width() as usize;
    // How many bytes per pixel RGBA
    // frame.as_ptr();
    let step = size*4;
    // (Assuming square) Step thru buffer.
    for i in 0..size*2 {
        let j = i*step;
        // Slices non-inclusively, ie: Even though j(n)+step = j(n+1) the ranges don't overlap. It take from..until
        let pre_slice = &buf[j..j+step];
        // Remove the last 12 bytes, alpha?
        let slice = &pre_slice[0..size*3];
        // let lol = &buf[j..j+step][0..size*3];

        // Skip odd lines (they are filled with zeros)
        if i & 1 != 1 {
            // debug
            println!("{:?} || {}-{}", slice, j, j+step);
        }
    }
}

