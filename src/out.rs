use std::io::{stdout, Write};
use ffmpeg::format::Pixel;

#[allow(dead_code)]
pub enum LinesFormat {
    /// All lines go to the right.
    Right,
    /// The first line goes to the right, the next to the left.
    RightLeft,
    /// The first line goes to the left, the next to the right.
    LeftRight,
    /// All lines go to the left.
    Left,
}

/// Must use square images
pub fn print_square(frame: &ffmpeg::frame::Video, lines_format: LinesFormat) {
    if frame.format() != Pixel::RGB24 { panic!("Must use Pixel::RGB24"); }
    if frame.width() != frame.height() { panic!("Must be 1:1 aspect ratio"); } 
    // Not sure if you can stack multiple frames on top of each other or what,
    // but we just want the first one.
    let z = 0;

    let size = frame.width() as usize;
    // How many bytes per pixel RGBA
    let linesize = unsafe { std::ptr::addr_of!((*frame.as_ptr()).linesize) };
    let step = (unsafe { *linesize })[z] as usize;
    
    let mut stdout = stdout();
    let mut lock = stdout.lock();

    let buf = frame.data(z); 
    // (Assuming square) Step thru buffer.
    for i in 0..size {
        let j = i*step;
        // Slices non-inclusively, ie: Even though j(n)+step = j(n+1) the ranges don't overlap. It take from..until
        let pre_slice = &buf[j..j+step];
        // Only take the size*3 of bytes, ignoring alpha values which are stored after.
        // let slice = &pre_slice[0..size*3];
        let mut slice = Vec::from(&pre_slice[0..size*3]);
        // let lol = &buf[j..j+step][0..size*3];

        match lines_format {
            LinesFormat::Right => {/* Do nothing */},
            LinesFormat::RightLeft => if i & 1 == 1 { reverse(&mut slice) },
            LinesFormat::LeftRight => if i & 1 != 1 { reverse(&mut slice) },
            LinesFormat::Left => slice.reverse(),
        }


        // DEBUG
        // println!("{:?} || {}-{}", slice, j, j+step);
        // NORMAL
        lock.write(&slice).unwrap();
    }
    stdout.flush().unwrap();
}


/// Reverses the order of every grouping of 3 bytes.
fn reverse(raw: & mut [u8]) {
    // I would prefer this only went over the elements once,
    // not twice.

    let x = raw
        .iter()
        .array_chunks::<3>()
        .rev()
        .flatten()
        .map(|f| *f)
        .collect::<Vec<u8>>();
    
    x.iter().enumerate().for_each(|f| raw[f.0] = *f.1);
}