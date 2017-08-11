Rust HTTP Live Stream Library
------------------------------------

Repository forked from https://github.com/OpenAnsible/rust-hls (Thanks for LuoZijun)

Installing
----------

Add this repository to Cargo.toml, then run ``cargo install``:

.. code:: toml
  
  [dependencies]
  hls = { git = "https://github.com/kentyson/rust-hls" }


Example usage
-------------

.. code:: rust

    extern crate hls;

    use std::str::FromStr;
    use hls::Playlist;

    static Master_Playlist: &'static str = r#"#EXTM3U
    #EXT-X-VERSION:3
    #EXT-X-TARGETDURATION:12
    #EXT-X-MEDIA-SEQUENCE:1
    #EXTINF:12.1,Title 1
    media-b2000000_1.ts?wowzasessionid=2029972411
    #EXTINF:12.1,Title 2
    media-b2000000_2.ts?wowzasessionid=2029972411
    #EXTINF:12.1,
    media-b2000000_3.ts?wowzasessionid=2029972411
    #EXTINF:12.1,
    media-b2000000_4.ts?wowzasessionid=2029972411
    #EXTINF:12.1,
    media-b2000000_5.ts?wowzasessionid=2029972411
    #EXTINF:12.1,
    media-b2000000_6.ts?wowzasessionid=2029972411
    #EXTINF:12.1,
    media-b2000000_7.ts?wowzasessionid=2029972411
    #EXT-X-ENDLIST"#;

    fn main() {
        println!("Master Playlist:\n{}\n\n", Master_Playlist);
        let res = Playlist::from_str(Master_Playlist);
        assert_eq!(res.is_ok(), true);
    
        let playlist = res.unwrap();
        assert_eq!(Master_Playlist, playlist.to_string());
    }

