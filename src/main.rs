extern crate ffmpeg_next as ffmpeg;

use anyhow::Error;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::path::Path;
mod test;

fn main() -> Result<(), Error> {

    // instantiate ffmpeg
    ffmpeg::init()?;

    // read the input file
    if let Ok(mut ictx) = input(&Path::new("input")) {

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        // Ctx's video decoder
        let mut decoder = context_decoder.decoder().video()?;
        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        // closure
        let mut receive_and_process_decoded_frames = |decoder: &mut ffmpeg::decoder::Video| -> Result<(), Error> {
                // get empty frame
                let mut decoded = Video::empty();
                // recieve next frame into decoded
                while decoder.receive_frame(&mut decoded).is_ok() {
                    // get another empty frame
                    let mut rgb_frame = Video::empty();
                    // scale image? Not sure what this does bit it doesn't work without it.
                    scaler.run(&decoded, &mut rgb_frame)?;

                    // test::print_raw(&rgb_frame.data(0));
                    // test::print_square(&rgb_frame.data(0), rgb_frame.width() as usize);
                    test::print_square(&rgb_frame);
                    // test::small_matrix();
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            // else what?
            // Is this for multiple video streams?
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    Ok(())
}

