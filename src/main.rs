mod support;
mod Duration;

fn main() {

    let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";
    //Display the content of the m3u8 file
    support::display_content_of_m3u8_url(url);
    //Calculate the duration of the whole video
    Duration::from_secs_f32(url);

}
