#![feature(test)]

extern crate test;
extern crate hls;

use std::str::FromStr;

use test::Bencher;
use hls::{Playlist};

#[allow(non_upper_case_globals)]
static Master_Playlist: &'static str = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=300000
chunklist-b300000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=600000
chunklist-b600000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=850000
chunklist-b850000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1000000
chunklist-b1000000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1500000
chunklist-b1500000.m3u8"#;

#[bench]
fn bench_m3u8_parse(b: &mut Bencher) {
    b.iter(||{
        let _ = Playlist::from_str(Master_Playlist);
    });
}
