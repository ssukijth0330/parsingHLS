mod duration;

fn main() {

    let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";
    // let url = "http://localhost/hls/bigb_720.m3u8";
    //let url = "http://localhost/hls/bigb_480.m3u8";
    //let url = "https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8";
    //let url = "http://localhost/hls/sintel/sintel_4k.m3u8";
    println!("URL: {}\n", url);
    // Display the content of the m3u8 file
    // duration::display_content_of_m3u8_url(url);
    // Calculate the duration of the whole video
    duration::parse_m3u8_url(url);
}
