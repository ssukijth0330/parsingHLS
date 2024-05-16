use core::{f32, str};
use reqwest::blocking::get;

//function to parse the HLS m3u8 file to get duration by using Duration::from_secs_f32
// Input m3u8 URL
// Output print the duration of the video
pub fn from_secs_f32(url: &str) {
    //read the url string. 
    let response = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");
    //split the file by new line
    let lines: Vec<&str> = response.split("\n").collect();
    let mut duration:f32 = 0.0;
    // To calulate the duration of the entire video in the manifest
    // The code needs to parse the #EXTINF tag and sum the duration of each segment
    // such as (sec)
    // #EXTINF:10.00,
    // abc_1.ps
    // #EXTINF:12.01,
    // abc_2.ps
    //
    // So the total duration of the video is 10.00 + 12.01 = 22.01 sec
    // The code will print the total duration of the video
    for line in lines {
        if line.contains("#EXTINF") {
            //let duration_str = line.split(":").collect::<Vec<&str>>()[1];
            let duration_vec: Vec<&str> = line.split(":").collect();
            //get the duration but it is in a string format
            // but it may contain unwanted characters
            let duration_f32_str_raw = duration_vec[1].to_string();
            // trim the string
            let duration_f32_str_raw = duration_f32_str_raw.trim();
            // There is a "," at the end of the duration string
            // remove the "," from the string
            let remove_char = ',';
            // remove the remove_char from the string
            let duration_f32_str: String = remove_char_from_string(&duration_f32_str_raw, remove_char);

            // convert the string to f32 format
            let duration_f32: f32 = duration_f32_str.parse().unwrap();
            //sum the duration of each segment
            duration = duration + duration_f32;
        }
    }
    println!("The duration to play all video segments in this file is: {}", duration
);
}

//function remove all characters from a string
pub fn remove_char_from_string(s: &str, c: char) -> String {
    s.chars().filter(|&x| x != c).collect()
}
