use reqwest::blocking::get;

// function to parse the HLS m3u8 file to get EXT-X-STREAM-INF tag
// Input m3u8 file path
// Output string with the EXT-X-STREAM-INF tag
// Example:
// #EXT3U
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
//
// The function will return the line with #EXT-X-STREAM-INF tag and the file name next to it.
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// Then it will read the file bigb_480.m3u8 and print the content of the file.
// ...
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
// Then it will read the file bigb_720.m3u8 and print the content of the file.
// ...
//
pub fn parse_m3u8_file(file_path: &str) {
    //read the file_path string. 
    let file = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    //split the file by new line
    let lines: Vec<&str> = file.split("\n").collect();
    //get the last line of the file
    let last_line = lines[lines.len()-2];
    let mut next_line = false;
    for line in lines {
        if line.contains("#EXT-X-STREAM-INF") {
            println!("{}", line);
            next_line = true;
            continue
        }
        if next_line {
            println!("{}", line);
            next_line = false;
            println!("The file, {} contains the following segment files", line);
            read_file(line);
        }
    }
}

//fuction typing content of a file
pub fn read_file(file_path: &str) {
    println!("Reading the file: {}", file_path);
    //read the file_path string. 
    let file = std::fs::read_to_string(file_path)
        .expect("Failed to read the file");
    println!("{}", file);
}

//function to read URL of m3u8 and parse manifest in m3u8 find the EXT-X-STREAM-INF tag and follow the file
// Input m3u8 URL
// Output string with the EXT-X-STREAM-INF tag
// Example:
// #EXT3U
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
//
// The function will return the line with #EXT-X-STREAM-INF tag and the file name next to it. 
// Then it will read the file and print the content of the file.
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=232370,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=640x480
// bigb_480.m3u8
// Then it will read the file bigb_480.m3u8 and print the content of the file.
// ...
// #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=649879,CODECS="mp4a.40.2,avc1.4d4015",RESOLUTION=1024x720
// bigb_720.m3u8
// Then it will read the file bigb_720.m3u8 and print the content of the file.
// ...
//
pub fn parse_m3u8_url(url: &str) {
    //read the url string. 
    display_content_of_m3u8_url(url);

    println!("Parse m3u8 url:, {}", url);
    let file = get(url)
        .expect("Failed to read the URL")
        .text()
        .expect("Failed to read the URL");
    //split the file by new line
    let lines: Vec<&str> = file.split("\n").collect();
    let mut next_line = false;
    for line in lines {
        if line.contains("#EXT-X-STREAM-INF") {
            println!("{}", line);
            next_line = true;
            continue
        }
        if next_line { // follow the file
            println!("{}", line);
            next_line = false;
            println!("The file, {} contains the following segments", line);

            //remove the last "/" from the url
            let url_removed_file = url.trim_end_matches("/");

            //create url of the file
            let url = format!("{}/{}", url_removed_file, line);
            println!("The URL of the file is: {}", url);

            display_content_of_m3u8_url(&url);
        }
    }
}

//function to read URL and print the content
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