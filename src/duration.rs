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

    //split the content line-by-line and put into vector
    let lines: Vec<&str> = file.split("\n").collect();
    let mut read_inside_m3u8 = false;
    for line in lines {
        if read_inside_m3u8 { // Prevoiuse line contains #EXT-X-STREAM-INF
            if line.contains(".m3u8") {
                // got "line = bigb_480.m3u8" for example
                // replace filename of the original URL with the inside m3u8 file name
                // For example:
                // "http://hostname/path/bigb.m3u8"  <---- "bigb_480.m3u8"
                let url = replace_with_newfilename_in_org_url(url.trim(), line.trim());
                // Calculate the sum of duration of the video of that bandwidth
                let duration = from_secs_f32(&url);
                println!("The duration of the video of this bandwidth,({}) is: {}\n", line, duration);
            } else {
                println!("Warning: expected m3u8 file, but got: {}", line);
            }
            //set the flag to false
            read_inside_m3u8 = false;
            continue;
        }
        match line.to_string() {
            s if s.contains("#EXT-X-STREAM-INF") => { // there is a second layer of m3u8
                // #EXT-X-STREAM-INF:,RESOLUTION=640x480
                // bigb_480.m3u8
                //
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
            //got the duration in duration_vec[1]="10.00,"
            // but it may contain unwanted characters
            // convert the duration to string
            let duration_f32_str_raw = duration_vec[1].to_string();
            // trim the string, take out leading/end white spaces
            let duration_f32_str_raw = duration_f32_str_raw.trim();
            // remove ending "," from the string
            //by split the string with "," delimiter
            let duration_f32_vec: Vec<&str> = duration_f32_str_raw.split(",").collect();
            // The duration is in duration_f32_vec[0]="10.00"
            // then convert it to f32 format
            let duration_f32: f32 = duration_f32_vec[0].parse().unwrap();
            //sum duration of each segment
            duration = duration + duration_f32;
        } // more HLS tags can be added here
    }
    duration
}

// SUPPORT FUNCTIONS:
//
//Replace filename inside the original URL with the file name that we found in the m3u8 file
//
//Input: the original URL, the file name that we found in the m3u8 file
//Output: the combination of base URL with path and the file name that we found in the m3u8 file
//
// "http://hostname/path/bigb.m3u8"  <---- "bigb_480.m3u8"
pub fn replace_with_newfilename_in_org_url(in_url: &str, m3u8_filename: &str) -> String {
    //split the file name out from the original URL
    // "http://hostname/path/bigb.m3u8"
    let file_name = in_url.split('/').last().unwrap();
    //Replace the file name in the origianl URL with blank
    let url_no_file = in_url.replace(file_name, "");
    // Comcantinate the output URL with the file name that we found in the m3u8 file
    // "http://hostname/path/" + "bigb_480.m3u8"
    let url = format!("{}{}", url_no_file, m3u8_filename);
    url
}

//Read URL and print the content
#[allow(dead_code)]
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

// Test suite
//
#[cfg(test)]
mod tests {
    //Bring the functions from the outer scope into the test scope
    use super::*;

    #[test]
    fn duration_test() {
        // Single Level of m3u8 file:
        let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";
        // The entire video duration is 100.96
        assert_eq!(from_secs_f32(url), 100.96);
    }

    #[test]
    fn duration_test_negative_test_case() {
        // Single Level of m3u8 file:
        let url = "https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8";
        // The entire video duration is 100.96
        assert_ne!(from_secs_f32(url), 101.96);
    }
}
