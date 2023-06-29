use std::io::{stdout, Write};
use ffmpeg::format::Pixel;

/// Must use square images
// pub fn print_square(buf: &[u8], width_hight: usize, line) {
pub fn print_square(frame: &ffmpeg::frame::Video) {
    if frame.format() != Pixel::RGB24 { panic!("Must use Pixel::RGB24"); }
    if frame.width() != frame.height() { panic!("Must be square aspect ratio"); } 
    // Not sure if you can stack multiple frames on top of each other or what,
    // but we just want the first one.
    let z = 0;

    let buf = frame.data(z); 
    let size = frame.width() as usize;
    // How many bytes per pixel RGBA
    let linesize = unsafe { std::ptr::addr_of!((*frame.as_ptr()).linesize) };
    let step = (unsafe { *linesize })[z] as usize;
    
    let mut stdout = stdout();
    let mut lock = stdout.lock();

    // (Assuming square) Step thru buffer.
    for i in 0..size {
        let j = i*step;
        // Slices non-inclusively, ie: Even though j(n)+step = j(n+1) the ranges don't overlap. It take from..until
        let pre_slice = &buf[j..j+step];
        // Only take the size*3 of bytes, ignoring alpha values which are stored after.
        let slice = &pre_slice[0..size*3];
        // let lol = &buf[j..j+step][0..size*3];

        // DEBUG
        // println!("{:?} || {}-{}", slice, j, j+step);
        // NORMAL
        lock.write(slice).unwrap();
    }
    stdout.flush().unwrap();
}

