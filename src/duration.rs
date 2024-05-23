use core::{f32, str};
use reqwest::blocking::get;

// THE KEY FUNCIONS:
//
//Parsing the HLS tag and calculate the sum the duration of each segments:
//
// If the master playlist contains nested m3u8 files (tag #EXT-X-STREAM-INF)
// then read the nested m3u8 file and calculate the duration of the video
//
// If the master playlist contains only one level of m3u8 files (tag #EXTINF)
// then calculate the duration of the video
//
// EXAMPLE of Two level of m3u8 files.
// ...
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
// ...
// 
// EXAMPLE of Single LEVEL of m3u8 files
// ...
// #EXTINF:10.00,
// ...
//
// Input: URL
// Output: call the next function to calulate the sum of duration of the video
//
pub fn parse_m3u8_url(url: &str) {
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
            // got "line = bigb_480.m3u8"
            // replace filename of the original URL with the inside m3u8 file name
            // "http://hostname/path/bigb.m3u8"  <---- "bigb_480.m3u8"
            let url = replace_with_newfilename_in_org_url(url.trim(), line.trim());
            // then the new URL is "http://hostname/path/bigb_480.m3u8"

            // Calculate the sum of duration of the video of that bandwidth
            let duration = from_secs_f32(&url);
            println!("The duration of the video of this bandwidth,({}) is: {}\n", line, duration);
            continue;
        }
        match line.to_string() {
            s if s.contains("#EXT-X-STREAM-INF") => { // there is a second layer of m3u8
                // set "read_inside_m3u8 to true"
                read_inside_m3u8 = true;
                continue;
            },
            s if s.contains("#EXTINF") => { // this is the durration tag so there is no second layer of m3u8
                // Single layer of m3u8:
                // then calculate the sum of duration of the video
                let duration = from_secs_f32(url);
                println!("The duration of the video is(secs.milisecs): {}\n", duration);
                break;
            },
            _ => { continue; }

         }
    }
}

//function to parse the HLS m3u8 file to get duration of each segment, 
//then sum the duration of each segment to get the total duration of the video
//
// Input:  m3u8 URL
// Output:  print the duration of the total duration of the video
//
pub fn from_secs_f32(url: &str) -> f32{
    //read the content of the URL
    let response = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");
    //Split the content into vector, line by line
    let lines: Vec<&str> = response.split("\n").collect();
    let mut duration:f32 = 0.0;
    //println!("Calculating duration of video, {}", url);
    // Combine duration of segments to get the total duration of the video:
    // ...
    // #EXTINF:10.00,
    // abc_1.ps
    // #EXTINF:12.01,
    // abc_2.ps
    // ...
    for line in lines {
        if line.contains("#EXTINF") {  // found #EXTINF:10.00,
            //  split the string by using ":" into vector
            let duration_vec: Vec<&str> = line.split(":").collect();
            //got the duration in duration_vec[1]
            // but it may contain unwanted characters
            // convert the duration to string
            let duration_f32_str_raw = duration_vec[1].to_string();
            // trim the string, take out leading/end white spaces
            let duration_f32_str_raw = duration_f32_str_raw.trim();
            // remove ending "," from the string
            //by split the string with "," delimiter
            let duration_f32_vec: Vec<&str> = duration_f32_str_raw.split(",").collect();
            // The duration is in duration_f32_vec[0]
            // then convert it to f32 format
            let duration_f32: f32 = duration_f32_vec[0].parse().unwrap();           

            //sum duration of each segment
            duration = duration + duration_f32;
        }
    }
    duration
    // Warning:
    // println!("Note:
    // The calculated duration may not accurately reflect real-time video playback. In adative bitrate streaming, 
    // bandwidth fluctuation can result in the display of video segments from different bandwidths. 
    // The duration above is calculated based on segments from the same bandwith m3u8 file.
    //
    // To solve this problem, it necessary to ensure that each video segment maintains a consistent duration across all bandwidth.");
}

// SUPPORT FUNCTIONS:
//
//Replace filename inside the original URL with the file name that we found in the m3u8 file
//
//Input: the original URL, the file name that we found in the m3u8 file
//Output: the combination of base URL with path and the file name that we found in the m3u8 file
//
pub fn replace_with_newfilename_in_org_url(in_url: &str, m3u8_filename: &str) -> String {
    //split the file name out from the original URL
    let file_name = in_url.split('/').last().unwrap();
    //Replace the file name in the origianl URL with blank
    let url_no_file = in_url.replace(file_name, "");
    //combine output URL with the file name that we found in the m3u8 file
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