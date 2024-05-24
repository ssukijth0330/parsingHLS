Parsing HLS project
===================
Assuming before start:
- Cargo is already installed on PC.
- VScode is already installed with RUST extension.
- Internet access to github.

Limitation:
- The project is NOT designed to validate the HLS manifest tags. It only focus of two M3U tag, #EXTINF and #EXT-X-STREAM-INF, 
and then calculate the total duration of video segments to get the entire video playback time.

Please note: 
The entire video playback duration is calculated from a single m3u8 file.


1) Load the code:
    - From the terminal of VScode and type, 
        'git clone https://github.com/ssukijth0330/parsingHLS'
2) Build
    - From the terminal of VScode and type, 
        'cd parsingHLS', 
        'cargo build'
3) Run
    - From the terminal of VScode and type, 
        'cargo run'

        or run the binary file.

        'cd terget/debug',
        './parsingHLS'

4) Test suit: Test the duration parser function
    - From the terminal of VScode and type, 
        'cargo test'


Output from running the application (cargo run)
===============================================

Output of single level of manifest
----------------------------------
URL: https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8

The duration to play all segments of this video is(secs): 100.96




Output of two level of manifest
-------------------------------
URL: https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8

Calculating duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1200k/vod.m3u8
The duration to play all segments of this video is(secs): 462.83722

Calculating duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1800k/vod.m3u8
The duration to play all segments of this video is(secs): 462.83722
...


Ref:
----
Sample of single level m3u8
--------------------------
.
.
.
#EXTINF:12.166,
#EXT-X-BYTERANGE:1430680@4048392
segment_1440468394459_1440468394459_1.ts
#EXTINF:13.292,
#EXT-X-BYTERANGE:840360@5479072
segment_1440468394459_1440468394459_1.ts
.
.
.

Sample of nested m3u
--------------------

https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/master_unenc_avc_aac_subs_ccjk.m3u8"
.
.
.
#EXT-X-STREAM-INF:BANDWIDTH=1977345,AVERAGE-BANDWIDTH=1318326,CODECS="mp4a.40.2,avc1.64001f",RESOLUTION=853x480,FRAME-RATE=23.97,AUDIO="aac-128k",SUBTITLES="subs"
avc/unenc/1200k/vod.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=2877522,AVERAGE-BANDWIDTH=1923669,CODECS="mp4a.40.2,avc1.64001f",RESOLUTION=853x480,FRAME-RATE=23.97,AUDIO="aac-128k",SUBTITLES="subs"
avc/unenc/1800k/vod.m3u8
.
.
.

The code will goto the next level of m3u8 file and calculate the duration of https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1200k/vod.m3u8 and https://lw.bamgrid.com/2.0/hls/vod/bam/ms02/hls/dplus/bao/avc/unenc/1800k/vod.m3u8


Documents:
M3U (MP3 URL) tag:     https://en.wikipedia.org/wiki/M3U#Extended_M3U
Playlist:  https://datatracker.ietf.org/doc/html/rfc8216#section-4
Sample of HLS video: https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8

