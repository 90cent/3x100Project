#[get("/ytdl/<url_or_id>")]
pub fn ytdl(url_or_id: String) -> Result<String,std::io::Error> {
    let ytvid_handle = if url_or_id.contains("https://www.youtube.com/watch?v=") {
        let url = url_or_id;
        YouTubeVid::from_url(url)
    }
    else {
        let id = url_or_id;
        YouTubeVid::from_id(id)
    };


    Ok(format!("{}:{}",ytvid_handle.id,ytvid_handle.url))
}

pub struct YouTubeVid {
    id: String,
    url: String
}

impl YouTubeVid {
    pub fn from_url(_url: String) -> YouTubeVid {
        let _id = _url.replace("https://www.youtube.com/watch?v=", "");

        YouTubeVid {id: _id, url: _url}
    }

    pub fn from_id(_id: String) -> YouTubeVid {
        let _url = format!("https://www.youtube.com/watch?v={}",_id);

        YouTubeVid {id: _id, url: _url}
    }

    // Examples: https://www.youtube.com/watch?v=g-_hVXzkn0o, https://www.youtube.com/watch?v=hPrMtIXUh1s
}