mod support;
mod Duration;

fn main() {

    let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";
    support::display_content_of_m3u8_url(url);
    Duration::from_secs_f32(url);

}
