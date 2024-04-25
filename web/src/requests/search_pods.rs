use std::collections::HashMap;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Error;
use rss::Channel;
use wasm_bindgen::JsValue;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Deserialize, Debug)]
pub struct RecentEps {
    pub episodes: Vec<Episode>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct PodcastSearchResult {
    pub status: Option<String>, // for PodcastIndex
    pub resultCount: Option<i32>, // for iTunes
    pub feeds: Option<Vec<Podcast>>, // for PodcastIndex
    pub results: Option<Vec<ITunesPodcast>>, // for iTunes
}

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct UnifiedPodcast {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) url: String,
    #[allow(non_snake_case)]
    pub(crate) originalUrl: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) author: String,
    #[allow(non_snake_case)]
    pub(crate) ownerName: String,
    pub(crate) image: String,
    pub(crate) artwork: String,
    #[allow(non_snake_case)]
    pub(crate) lastUpdateTime: i64,
    pub(crate) categories: Option<HashMap<String, String>>,
    pub(crate) explicit: bool,
    #[allow(non_snake_case)]
    pub(crate) episodeCount: i32,
}

// Implement conversions from Podcast and ITunesPodcast to UnifiedPodcast
impl From<Podcast> for UnifiedPodcast {
    fn from(podcast: Podcast) -> Self {
        UnifiedPodcast {
            id: podcast.id,
            title: podcast.title,
            url: podcast.url,
            originalUrl: podcast.originalUrl,
            author: podcast.author,
            ownerName: podcast.ownerName,
            description: podcast.description,
            image: podcast.image, // Assuming artwork is the image you want to use
            link: podcast.link,
            artwork: podcast.artwork,
            lastUpdateTime: podcast.lastUpdateTime,
            categories: podcast.categories,
            explicit: podcast.explicit,
            episodeCount: podcast.episodeCount,
        }
    }
}

impl From<ITunesPodcast> for UnifiedPodcast {
    fn from(podcast: ITunesPodcast) -> Self {
        let genre_map: HashMap<String, String> = podcast.genres.into_iter().enumerate()
            .map(|(index, genre)| (index.to_string(), genre))
            .collect();

        let parsed_date = DateTime::parse_from_rfc3339(&podcast.releaseDate)
            .map(|dt| dt.timestamp())
            .unwrap_or(0);  // Default to 0 or choose a more sensible default


        UnifiedPodcast {
            id: podcast.trackId,
            title: podcast.trackName,
            url: podcast.feedUrl.clone(),
            originalUrl: podcast.feedUrl,
            author: podcast.artistName.clone(),
            ownerName: podcast.artistName,
            description: String::from("Descriptions not provided by iTunes"),
            image: podcast.artworkUrl100.clone(),
            link: podcast.collectionViewUrl,
            artwork: podcast.artworkUrl100,
            lastUpdateTime: parsed_date,
            categories: Some(genre_map),
            explicit: match podcast.collectionExplicitness.as_str() {
                "explicit" => true,
                "notExplicit" => false,
                _ => false,
            },
            episodeCount: podcast.trackCount.unwrap_or(0),
            // Map other fields as necessary
        }
    }
}



#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Podcast {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) url: String,
    #[allow(non_snake_case)]
    pub(crate) originalUrl: String,
    pub(crate) link: String,
    pub(crate) description: String,
    pub(crate) author: String,
    #[allow(non_snake_case)]
    pub(crate) ownerName: String,
    pub(crate) image: String,
    pub(crate) artwork: String,
    #[allow(non_snake_case)]
    pub(crate) lastUpdateTime: i64,
    pub(crate) categories: Option<HashMap<String, String>>,
    pub(crate) explicit: bool,
    #[allow(non_snake_case)]
    pub(crate) episodeCount: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct ITunesPodcast {
    pub wrapperType: String,
    pub kind: String,
    pub collectionId: i64,
    pub trackId: i64,
    pub(crate) artistName: String,
    pub(crate) trackName: String,
    pub(crate) collectionViewUrl: String,
    pub(crate) feedUrl: String,
    pub(crate) artworkUrl100: String,
    pub(crate) releaseDate: String,
    pub(crate) genres: Vec<String>,
    pub(crate) collectionExplicitness: String,
    pub(crate) trackCount: Option<i32>,

    // add other fields as needed
}

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct Episode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub links: Vec<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_length: Option<String>,
    pub artwork: Option<String>,
    pub content: Option<String>,
    pub authors: Vec<String>,
    pub guid: String,
    pub duration: Option<String>
}

#[derive(Deserialize, Debug, PartialEq, Clone, Serialize)]
pub struct PodcastFeedResult {
    // ... other fields ...
    pub(crate) episodes: Vec<Episode>,
}

pub async fn call_get_podcast_info(podcast_value: &String, search_api_url: &Option<String>, search_index: &str) -> Result<PodcastSearchResult, anyhow::Error> {
    let url = if let Some(api_url) = search_api_url {
        format!("{}?query={}&index={}", api_url, podcast_value, search_index)
    } else {
        return Err(anyhow::Error::msg("API URL is not provided"));
    };

    let response = Request::get(&url).send().await.map_err(|err| anyhow::Error::new(err))?;

    if response.ok() {
        let response_text = response.text().await.map_err(|err| anyhow::Error::new(err))?;

        let search_results: PodcastSearchResult = serde_json::from_str(&response_text)?;
        // web_sys::console::log_1(search_results.clone());
        Ok(search_results)
    } else {
        Err(anyhow::Error::msg(format!("Failed to fetch podcast info: {}", response.status_text())))
    }
}



pub async fn test_connection(search_api_url: &Option<String>) -> Result<(), Error> {
    let url = search_api_url.as_ref().ok_or_else(|| Error::msg("API URL is missing"))?;

    match Request::get(url).send().await {
        Ok(response) => {
            if response.ok() {
                Ok(())
            } else {
                Err(Error::msg(format!("HTTP error occurred: {}", response.status_text())))
            }
        }
        Err(err) => Err(Error::new(err)),
    }
}

// pub async fn call_parse_podcast_url(podcast_url: &str) -> Result<PodcastFeedResult, Error> {
//     let response_text = Request::get(podcast_url).send().await?.text().await?;
//     let channel = Channel::read_from(response_text.as_bytes())?;

//     // Fallback to podcast's main artwork if individual episode artwork is not available
//     let podcast_artwork_url = channel.image().map(|img| img.url().to_string())
//         .or_else(|| channel.itunes_ext().and_then(|ext| ext.image()).map(|url| url.to_string()));

//     let episodes = channel.items().iter().map(|item| {
//         let episode_artwork_url = item.itunes_ext().and_then(|ext| ext.image()).map(|url| url.to_string())
//             .or_else(|| podcast_artwork_url.clone());
//         let audio_url = item.enclosure().map(|enclosure| enclosure.url().to_string());
//         let itunes_extension = item.itunes_ext();
//         let duration = itunes_extension.and_then(|ext| ext.duration()).map(|d| d.to_string());
//         let description = if let Some(encoded_content) = item.content() {
//             Option::from(encoded_content.to_string())
//         } else {
//             Option::from(item.description().unwrap_or_default().to_string())
//         };
//         Episode {
//             title: Option::from(item.title().map(|t| t.to_string()).unwrap_or_default()),
//             description,
//             content: item.content().map(|c| c.to_string()),
//             enclosure_url: audio_url,
//             enclosure_length: item.enclosure().map(|e| e.length().to_string()),
//             pub_date: item.pub_date().map(|p| p.to_string()),
//             authors: item.author().map(|a| vec![a.to_string()]).unwrap_or_default(),
//             links: item.link().map(|l| vec![l.to_string()]).unwrap_or_default(),
//             artwork: episode_artwork_url,
//             guid: item.title().map(|t| t.to_string()).unwrap_or_default(),
//             duration
//         }
//     }).collect();

//     let feed_result = PodcastFeedResult {
//         episodes,
//     };

//     Ok(feed_result)
// }

pub async fn call_parse_podcast_url(server_name: String, api_key: &Option<String>, podcast_url: &str) -> Result<PodcastFeedResult, Error> {
    let encoded_podcast_url = urlencoding::encode(podcast_url);
    let endpoint = format!("{}/api/data/fetch_podcast_feed?podcast_feed={}", server_name, encoded_podcast_url);
    
    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    let request = Request::get(&endpoint)
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key_ref)
        .send()
        .await?;

    if request.ok() {
        let response_text = request.text().await?;
        let channel = Channel::read_from(response_text.as_bytes())?;

        let podcast_artwork_url = channel.image().map(|img| img.url().to_string())
            .or_else(|| channel.itunes_ext().and_then(|ext| ext.image()).map(|url| url.to_string()));

        let episodes = channel.items().iter().map(|item| {
            Episode {
                title: Option::from(item.title().map(|t| t.to_string()).unwrap_or_default()),
                description: Option::from(item.description().unwrap_or_default().to_string()),
                content: item.content().map(|c| c.to_string()),
                enclosure_url: item.enclosure().map(|enclosure| enclosure.url().to_string()),
                enclosure_length: item.enclosure().map(|e| e.length().to_string()),
                pub_date: item.pub_date().map(|p| p.to_string()),
                authors: item.author().map(|a| vec![a.to_string()]).unwrap_or_default(),
                links: item.link().map(|l| vec![l.to_string()]).unwrap_or_default(),
                artwork: item.itunes_ext().and_then(|ext| ext.image()).map(|url| url.to_string())
                    .or_else(|| podcast_artwork_url.clone()),
                guid: item.guid().map(|g| g.value().to_string()).unwrap_or_default(),
                duration: item.itunes_ext().and_then(|ext| ext.duration()).map(|d| d.to_string())
            }
        }).collect();

        Ok(PodcastFeedResult { episodes })
    } else {
        Err(anyhow::Error::msg(format!("Failed to fetch podcast feed: HTTP {}", request.status())))
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PodcastInfo {
    pub title: String,
    pub description: String,
    pub artwork_url: Option<String>,
    pub author: String,
    pub website: String,
    pub categories: Vec<String>,
    pub explicit: bool,
    pub episode_count: i32,
}

pub async fn call_parse_podcast_channel_info(podcast_url: &str) -> Result<PodcastInfo, Error> {
    let response_text = Request::get(podcast_url).send().await?.text().await?;
    let channel = Channel::read_from(response_text.as_bytes())?;

    let podcast_artwork_url = channel.image().map(|img| img.url().to_string())
        .or_else(|| channel.itunes_ext().and_then(|ext| ext.image()).map(|url| url.to_string()));
    let podcast_title = channel.title().to_string();
    let podcast_description = channel.description().to_string();
    let podcast_authors = channel.itunes_ext().and_then(|ext| ext.author()).map(|a| a.to_string()).unwrap_or_default();
    let podcast_website = channel.link().to_string();
    let podcast_categories = channel.categories().iter().map(|c| c.name().to_string()).collect::<Vec<_>>();
    let podcast_explicit = channel.itunes_ext().map_or(false, |ext| ext.explicit().map(|e| e.eq("yes") || e.eq("true")).unwrap_or_default());

    let podcast_episode_count = channel.items().len() as i32;

    
    // Note: Add other podcast-level details as needed.

    let podcast_info = PodcastInfo {
        title: podcast_title,
        description: podcast_description,
        artwork_url: podcast_artwork_url,
        author: podcast_authors,
        website: podcast_website,
        categories: podcast_categories,
        explicit: podcast_explicit,
        episode_count: podcast_episode_count,
        // Include other fields as necessary.
    };

    Ok(podcast_info)
}


// In Databases

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub search_term: String,
    pub user_id: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct SearchResponse {
    pub data: Vec<SearchEpisode>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct SearchEpisode {
    pub PodcastID: i32,
    pub PodcastName: String,
    pub ArtworkURL: String,
    pub Author: String,
    pub Categories: String, // or change to appropriate type if you plan to parse the categories
    pub Description: String,
    pub EpisodeCount: i32,
    pub FeedURL: String,
    pub WebsiteURL: String,
    pub Explicit: i32, // or bool if it always contains 0 or 1
    pub UserID: i32,
    pub EpisodeID: i32,
    pub EpisodeTitle: String,
    pub EpisodeDescription: String,
    pub EpisodeURL: String,
    pub EpisodeArtwork: String,
    pub EpisodePubDate: String,
    pub EpisodeDuration: i32,
    // Existing fields
    pub ListenDuration: Option<i32>,
}

pub async fn call_search_database (
    server_name: &String, 
    api_key: &Option<String>, 
    request_data: &SearchRequest
) -> Result<Vec<SearchEpisode>, Error> {
    let url = format!("{}/api/data/search_data", server_name);

    // Convert Option<String> to Option<&str>
    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    let request_body = serde_json::to_string(request_data).map_err(|e| anyhow::Error::msg(format!("Serialization Error: {}", e)))?;

    let response = Request::post(&url)
        .header("Api-Key", api_key_ref)
        .header("Content-Type", "application/json")
        .body(request_body)?
        .send()
        .await?;

        if !response.ok() {
            return Err(anyhow::Error::msg(format!("Failed to search database: {}", response.status_text())));
        }
    // Deserialize the response body into a SearchResponse
        let search_response: SearchResponse = response.json().await?;

        // Extract the vector of episodes from the SearchResponse
        let results = search_response.data;

        Ok(results)
}