# parsingHLS

Assuming:
- Cargo is already installed on PC.
- VScode is already installed with RUST extension.
- Internet to access github.

1) Load the code:
    - Use the terminal of VScode and type
        git clone https://github.com/ssukijth0330/parsingHLS
2) Build
    - Use the terminal of VScode and type
        cargo build
3) Run
    - Use the terminal of VScode and type
        cargo run


This is part of the output
--------------------------
...
#EXTINF:12.459,
#EXT-X-BYTERANGE:701616@806332
segment_1440468394459_1440468394459_2.ts
#EXTINF:14.000,
#EXT-X-BYTERANGE:931352@1507948
segment_1440468394459_1440468394459_2.ts
#EXTINF:19.292,
#EXT-X-BYTERANGE:1593676@2439300
segment_1440468394459_1440468394459_2.ts
#EXTINF:7.834,
#EXT-X-BYTERANGE:657812@4032976
segment_1440468394459_1440468394459_2.ts
#EXT-X-ENDLIST

The duration to play all video segments in this file is: 100.96

Note: 
The code is hardcode to load the content of m3u8 from, 
"https://docs.evostream.com/sample_content/assets/hls-bunny-rangerequest/bunny/playlist.m3u8".

The duration of video is the summation of floating number from the tag #EXTINF inside the m3u8 file.


