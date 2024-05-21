use core::{f32, str};
use reqwest::blocking::get;

// THE KEY FUNCIONS:
//
//function to parse the HLS m3u8 file to get duration of each segment:
//
// TWO LEVEL of m3u8 files: (found #EXT-X-STREAM-INF tag in the file)
//Read URL of m3u8 and parse manifest in m3u8 find the EXT-X-STREAM-INF tag and follow the file
// Input m3u8 URL
// Output string with the EXT-X-STREAM-INF tag
// Example:
// ...
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
// ...
// Then combine the original URL with the inner m3u8 file, 
// then call -> from_secs_f32(url) for bigb_480.m3u8 file
// and then call -> from_secs_f32(url) for bigb_720.m3u8
//
// SINGLE LEVEL of m3u8 files: (found the #EXTINF tag in the file)
//  call -> from_secs_f32(url);
//
pub fn parse_m3u8_url(url: &str) {
    //display_content_of_m3u8_url(url);
    //read content from the url.
    let file = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");

    //split the content line-by-line put into vector
    let lines: Vec<&str> = file.split("\n").collect();
    let mut read_inside_m3u8 = false;
    for line in lines {
        if read_inside_m3u8 { // Prevoiuse line contains #EXT-X-STREAM-INF
            read_inside_m3u8 = false;
            // got the file name inside the m3u8 file
            // Then combine the base URL with the file name from inside the m3u8 file
            let url = replace_with_newfilename_in_url(url.trim(), line.trim());

            // Display the content of the m3u8 file
            //display_content_of_m3u8_url(&url);

            // Calculate the duration of the whole video
            println!("Calculate duration of {}", url);
            from_secs_f32(&url);
        }
        match line.to_string() {
            s if s.contains("#EXT-X-STREAM-INF") => {
                // Second layer of m3u8:
                // if found m3u8 inside m3u8
                // read the next line to get the second layer m3u8 file name
                read_inside_m3u8 = true;
                continue;
            },
            s if s.contains("#EXTINF") => {
                // Single layer of m3u8:
                // No #EXT-X-STREAM-INF tag in the file, 
                // So it contains only one layer of m3u8 file
                // then calculate the duration of the video
                from_secs_f32(url);
                break;
            },
            _ => { continue; }

         }
    }
}

//function to parse the HLS m3u8 file to get duration of each segment, 
//then sum the duration of each segment to get the total duration of the video
// Input m3u8 URL
// Output print the duration of the total duration of the video
pub fn from_secs_f32(url: &str) {
    //Debug:
    //display_content_of_m3u8_url(url); 
    let response = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");
    //Brake the m3u8 file into vector, line by line
    let lines: Vec<&str> = response.split("\n").collect();
    let mut duration:f32 = 0.0;
    // Combine the duration of segments in the manifest:
    // parse the #EXTINF tag and sum the duration of each segment
    // such as (sec)
    // #EXTINF:10.00,
    // abc_1.ps
    // #EXTINF:12.01,
    // abc_2.ps
    //
    // So the total duration of the video is 10.00 + 12.01 = 22.01 sec
    for line in lines {
        //focus on the line that contains the #EXTINF tag for duration
        if line.contains("#EXTINF") {
            //split the line by using ":"
            let duration_vec: Vec<&str> = line.split(":").collect();
            //got the duration in duration_vec[1]
            // but it may contain unwanted characters
            // convert the duration to string
            let duration_f32_str_raw = duration_vec[1].to_string();
            // trim the string, take out leading/end white spaces
            let duration_f32_str_raw = duration_f32_str_raw.trim();
            // remove the "," from the string
            //split the string by ","
            let duration_f32_vec: Vec<&str> = duration_f32_str_raw.split(",").collect();
            // The duration is in duration_f32_vec[0]
            // then convert it to f32 format
            let duration_f32: f32 = duration_f32_vec[0].parse().unwrap();
            //Debug: print the duration of each segment
            //print!("{} + ", duration_f32);

            //sum duration of each segment
            duration = duration + duration_f32;
        }
    }
    println!("The duration to play all segments of this video is(secs.milisecs): {}\n", duration);
    // Warning:
    // println!("Note:
    // The duration may not accurate, because in the adative Bitrate Streaming, 
    // we can not guarantee bandwidth of traffic, so the video segments from a different bandwidth may be use to display, 
    // but the duration that shown above is calculated from all segments of the same bandwith m3u8 file.
    
    // To solve this problem, we need to ensure that each video segment has the same duration, 
    // for every bandwidth.");
}

// SUPPORT FUNCTIONS:
//
//Combine the base URL with the file name that we found in the m3u8 file
//Input: the original URL, and the file name that we found in the m3u8 file
//Output: the combination of base URL with path and the file name that we found in the m3u8 file
pub fn replace_with_newfilename_in_url(in_url: &str, m3u8_filename: &str) -> String {
    //get the file name from the original URL
    let file_name = in_url.split('/').last().unwrap();
    //remove the file name from the original URL
    let url_no_file = in_url.replace(file_name, "");
    //combine the URL with the file name that we found in the m3u8 file
    let url = format!("{}{}", url_no_file, m3u8_filename);
    url
}

//Read URL and print the content
pub fn display_content_of_m3u8_url(url: &str) {
    println!("Display the content of m3u8 file:, {}", url);
    //read the url string. 
    let response = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");
    //print the response
    println!("{}", response);
}