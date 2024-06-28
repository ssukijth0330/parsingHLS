mod duration;

fn main() {

    //Single Level of m3u8 file:
    //let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";

    //Two levels of m3u8 files:
    let url = "https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8";
    println!("URL: {}\n", url);
    // Display the content of the m3u8 file
    //duration::display_content_of_m3u8_url(url);
    // Calculate and print out the duration of the entire video
    duration::parse_m3u8_url(url);

}