use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    content: String,
    id: String,
    title: String,
}


#[derive(Debug, Deserialize)]
struct Feed {
    title: String,
    #[serde(default, rename = "entry")]
    entries: Vec<Entry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://old.reddit.com/r/lfg/new/.rss")?.text()?;
    let feed: Feed = quick_xml::de::from_str(&resp)?;
    println!("{:?}", feed);
    Ok(())
}
