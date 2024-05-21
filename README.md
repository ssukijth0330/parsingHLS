# parsingHLS
------------
Assuming:
- Cargo is already installed on PC.
- VScode is already installed with RUST extension.
- Internet to access github.

1) Load the code:
    - From the terminal of VScode and type
        git clone https://github.com/ssukijth0330/parsingHLS
2) Build
    - From the terminal of VScode and type
        cd parsingHLS
        cargo build
3) Run
    - From the terminal of VScode and type
        cargo run

output
------
URL: https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8

The duration to play all segments of this video is(secs): 100.96



Note:
The code is hardcode to load the content of m3u8 from, 
"https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8".

The duration of video is the summation of floating number from the tag #EXTINF inside the m3u8 file.

Note:
The code is able to parse one or two level of M3U8 file.
Such as
https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8"
#EXT-X-STREAM-INF:BANDWIDTH=1977345,AVERAGE-BANDWIDTH=1318326,CODECS="mp4a.40.2,avc1.64001f",RESOLUTION=853x480,FRAME-RATE=23.97,AUDIO="aac-128k",SUBTITLES="subs"
avc/unenc/1200k/vod.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=2877522,AVERAGE-BANDWIDTH=1923669,CODECS="mp4a.40.2,avc1.64001f",RESOLUTION=853x480,FRAME-RATE=23.97,AUDIO="aac-128k",SUBTITLES="subs"
avc/unenc/1800k/vod.m3u8

The code will goto the next level of m3u8 file and calculate the duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1200k/vod.m3u8
and https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1800k/vod.m3u8

The output of second level m3u8
-------------------------------
URL: https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8

Calculate duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1200k/vod.m3u8
The duration to play all segments of this video is(secs): 462.83722

Calculate duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1800k/vod.m3u8
The duration to play all segments of this video is(secs): 462.83722