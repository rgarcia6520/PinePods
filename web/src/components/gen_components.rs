use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::history::{BrowserHistory, History};
use crate::requests::search_pods::{call_get_podcast_info, test_connection};
use web_sys::{HtmlInputElement, window, MouseEvent};
use web_sys::HtmlSelectElement;
use yewdux::prelude::*;
use crate::components::context::{AppState};
use crate::components::episodes_layout::SafeHtml;
use yew::Callback;
use crate::requests::pod_req::{call_download_episode, call_queue_episode, call_save_episode, DownloadEpisodeRequest, Episode, EpisodeDownload, HistoryEpisode, QueuePodcastRequest, QueuedEpisode, SavePodcastRequest, SavedEpisode};
use crate::requests::search_pods::SearchEpisode;
use std::any::Any;

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    pub error_message: UseStateHandle<Option<String>>,
}


#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    // Your existing logic here...
    let error_message = use_state(|| None::<String>);
    let (state, dispatch) = use_store::<AppState>();

    {
        let error_message = error_message.clone();
        use_effect(move || {
            let window = window().unwrap();
            let document = window.document().unwrap();

            let error_message_clone = error_message.clone();
            let closure = Closure::wrap(Box::new(move |_event: Event| {
                error_message_clone.set(None);
            }) as Box<dyn Fn(_)>);

            if error_message.is_some() {
                document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            }

            // Return cleanup function
            move || {
                if error_message.is_some() {
                    document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
                }
                closure.forget(); // Prevents the closure from being dropped
            }
        });
    }

    let error_message_clone = error_message.clone();

    let on_error = {
        Callback::from(move |_: ()| {
            let error_message = error_message_clone.clone();

            wasm_bindgen_futures::spawn_local(async move {

            });
        })
    };

    if let Some(error) = props.error_message.as_ref() {
        html! {
            <div class="error-snackbar">{ error }</div>
        }
    } else {
        html! {}
    }
}

#[allow(non_camel_case_types)]
#[function_component(Search_nav)]
pub fn search_bar() -> Html {
    let history = BrowserHistory::new();
    let dispatch = Dispatch::<AppState>::global();
    let state: Rc<AppState> = dispatch.get();
    let podcast_value = use_state(|| "".to_string());
    let search_index = use_state(|| "podcast_index".to_string()); // Default to "podcast_index"
    let (app_state, dispatch) = use_store::<AppState>();

    let history_clone = history.clone();
    let podcast_value_clone = podcast_value.clone();
    let search_index_clone = search_index.clone();
    let dispatch_clone = dispatch.clone();
    // State for toggling the dropdown in mobile view
    let mobile_dropdown_open = use_state(|| false);
    let on_submit = {
        Callback::from(move |_: ()| {
            let api_url = state.server_details.as_ref().map(|ud| ud.api_url.clone());
            let history = history_clone.clone();
            let search_value = podcast_value_clone.clone();
            let search_index = search_index_clone.clone();
            let dispatch = dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                dispatch.reduce_mut(|state| state.is_loading = Some(true));
                let cloned_api_url = &api_url.clone();
                match test_connection(&cloned_api_url.clone().unwrap()).await {
                    Ok(_) => {
                        match call_get_podcast_info(&search_value, &api_url.unwrap(), &search_index).await {
                            Ok(search_results) => {
                                web_sys::console::log_1(&format!("results: {:?}", search_results).into());
                                dispatch.reduce_mut(move |state| {
                                    state.search_results = Some(search_results);
                                });
                                dispatch.reduce_mut(|state| state.is_loading = Some(false));
                                history.push("/pod_layout"); // Use the route path
                            },
                            Err(e) => {
                                dispatch.reduce_mut(|state| state.is_loading = Some(false));
                                web_sys::console::log_1(&format!("Error: {}", e).into());
                            }
                        }
                    },
                    Err(err_msg) => {
                        dispatch.reduce_mut(|state| state.is_loading = Some(false));
                        web_sys::console::log_1(&format!("Error: {}", err_msg).into());
                    }
                }
            });
        })
    };

    let on_input_change = {
        let podcast_value = podcast_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            podcast_value.set(input.value());
        })
    };

    let on_select_change = {
        let search_index = search_index.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                search_index.set(select.value());
            }
        })
    };

    let on_select_radio_change = {
        let search_index = search_index.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(radio) = e.target_dyn_into::<HtmlInputElement>() {
                search_index.set(radio.value());
            }
        })
    };
    

    let on_submit_click = {
        let on_submit = on_submit.clone(); // Clone the existing on_submit logic
        Callback::from(move |_: MouseEvent| {
            on_submit.emit(()); // Invoke the existing on_submit logic
        })
    };

    let on_search_click = {
        let on_submit = on_submit.clone();
        let mobile_dropdown_open = mobile_dropdown_open.clone();
        Callback::from(move |_: MouseEvent| {
            if web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() < 768.0 {
                mobile_dropdown_open.set(!*mobile_dropdown_open);
            } else {
                on_submit.emit(());
            }
        })
    };

    let prevent_default_submit = {
        let on_submit = on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent the default form submission
            on_submit.emit(());  // Emit the on_submit event
        })
    };


    let dropdown_open = use_state(|| false);

    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_: MouseEvent| {
            web_sys::console::log_1(&format!("Dropdown toggled: {}", !*dropdown_open).into()); // Log for debugging
            dropdown_open.set(!*dropdown_open);
        })
    };

    let on_dropdown_select = {
        let dropdown_open = dropdown_open.clone();
        let search_index = search_index.clone();
        move |category: &str| {
            search_index.set(category.to_string());
            dropdown_open.set(false);
        }
    };

    let on_dropdown_select_itunes = {
        let on_dropdown_select = on_dropdown_select.clone();
        Callback::from(move |_| on_dropdown_select("itunes"))
    };

    let on_dropdown_select_podcast_index = {
        let on_dropdown_select = on_dropdown_select.clone();
        Callback::from(move |_| on_dropdown_select("podcast_index"))
    };

    let toggle_mobile_dropdown = {
        let mobile_dropdown_open = mobile_dropdown_open.clone();
        Callback::from(move |_: MouseEvent| {
            mobile_dropdown_open.set(!*mobile_dropdown_open);
        })
    };



    html! {
    <div class="episodes-container w-full search-background"> // Ensure full width and set background color
        <form class="search-bar-container flex justify-end w-full mx-auto border-solid border-b-2 border-color" onsubmit={prevent_default_submit}>
            <div class="relative inline-flex"> // Set a max-width for the search bar content
                // Dropdown Button
                <button
                    id="dropdown-button"
                    onclick={toggle_dropdown}
                    class="hidden md:flex md:block flex-shrink-0 z-10 inline-flex items-center py-2.5 px-4 text-sm font-medium text-center text-gray-900 bg-gray-100 border border-r-0 border-gray-300 dark:border-gray-700 dark:text-white rounded-l-lg hover:bg-gray-200 focus:ring-4 focus:outline-none focus:ring-gray-300 dark:bg-gray-600 dark:hover:bg-gray-700 dark:focus:ring-gray-800"
                    type="button"
                >
                    {format!("{} ", (*search_index).as_str())}
                    // SVG icon
                    <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                    </svg>
                </button>
                // Dropdown Content
                {
                    if *dropdown_open {
                        html! {
                            <div class="search-dropdown-content-class absolute z-10 divide-y rounded-lg shadow">
                                <ul class="dropdown-container py-2 text-sm text-gray-700">
                                    <li class="dropdown-option" onclick={on_dropdown_select_itunes.clone()}>{ "iTunes" }</li>
                                    <li class="dropdown-option" onclick={on_dropdown_select_podcast_index.clone()}>{ "Podcast Index" }</li>
                                    // Add more categories as needed
                                </ul>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

            // Search Input Field
            // <div class="relative w-full">
                <input
                    type="search"
                    id="search-dropdown"
                    class="search-input block p-2.5 w-full z-20 text-sm rounded-r-lg border hidden md:inline-flex"
                    placeholder="Search"
                    required=true
                    oninput={on_input_change.clone()}
                />
            </div>
            // Search Button
            <button
                type="submit"
                class="search-btn p-2.5 text-sm font-medium rounded-lg border focus:ring-4 focus:outline-none"
                onclick={on_search_click.clone()}
            >
                    // SVG icon for search button
                    <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
                    </svg>
            </button>
            {
                // Mobile dropdown content
                if *mobile_dropdown_open {
                    html! {
                        <div class="search-drop absolute top-full right-0 z-10 divide-y rounded-lg shadow p-4">
                            // Outline buttons for podcast_index or itunes
                            <div class="inline-flex rounded-md shadow-sm" role="group">
                                <button
                                    type="button"
                                    class={format!("px-4 py-2 text-sm font-medium rounded-l-lg search-drop-button {}",
                                        if *search_index == "podcast_index" { "active" } else { "" })}
                                    onclick={on_dropdown_select_podcast_index}
                                >
                                    {"Podcast Index"}
                                </button>
                                <button
                                    type="button"
                                    class={format!("px-4 py-2 text-sm font-medium rounded-r-lg search-drop-button {}",
                                        if *search_index == "itunes" { "active" } else { "" })}
                                    onclick={on_dropdown_select_itunes}
                                >
                                    {"iTunes"}
                                </button>
                            </div>
                            // Text field for search
                            <input
                                type="text"
                                class="search-input shorter-input block p-2.5 w-full text-sm rounded-lg border"
                                placeholder="Search"
                                value={(*podcast_value).clone()}
                                oninput={on_input_change.clone()}
                            />
                            // Search button
                            <button class="search-btn no-margin border border-solid mt-4 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" onclick={on_submit_click.clone()}>
                                {"Search"}
                            </button>
                        </div>
                    }
                }
                else {
                    html! {}
                }
            }
        </form>
    </div>
}

}

#[derive(Properties, Clone)]
pub struct ContextButtonProps {
    pub episode: Box<dyn EpisodeTrait>,
}

#[function_component(ContextButton)]
pub fn context_button(props: &ContextButtonProps) -> Html {
    let search_index = use_state(|| "podcast_index".to_string());
    let dropdown_open = use_state(|| false);
    let (post_state, post_dispatch) = use_store::<AppState>();
    let api_key = post_state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let user_id = post_state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let server_name = post_state.auth_details.as_ref().map(|ud| ud.server_name.clone());


    let dropdown_ref = NodeRef::default();
    
    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_: MouseEvent| {
            web_sys::console::log_1(&format!("Dropdown toggled: {}", !*dropdown_open).into()); // Log for debugging
            dropdown_open.set(!*dropdown_open);
        })
    };

    
    let on_dropdown_select = {
        let dropdown_open = dropdown_open.clone();
        let search_index = search_index.clone();
        move |category: &str| {
            search_index.set(category.to_string());
            dropdown_open.set(false);
        }
    };

    let queue_api_key = api_key.clone();
    let queue_server_name = server_name.clone();

    // let server_name = server_name.clone();
    let on_add_to_queue = {
        let episode = props.episode.clone();
        Callback::from(move |_| {
            let server_name_copy = queue_server_name.clone();
            let api_key_copy = queue_api_key.clone();
            let request = QueuePodcastRequest {
                episode_title: episode.get_episode_title(),
                ep_url: episode.get_episode_artwork(),
                user_id: user_id.unwrap(), // replace with the actual user ID
            };
            let server_name = server_name_copy; // replace with the actual server name
            let api_key = api_key_copy; // replace with the actual API key
            let future = async move {
                let _ = call_queue_episode(&server_name.unwrap(), &api_key.flatten(), &request).await;
            };
            wasm_bindgen_futures::spawn_local(future);
            // dropdown_open.set(false);
        })
    };

    let saved_api_key = api_key.clone();
    let saved_server_name = server_name.clone();

    let on_save_episode = {
        let episode = props.episode.clone();
        Callback::from(move |_| {
            let server_name_copy = saved_server_name.clone();
            let api_key_copy = saved_api_key.clone();
            let request = SavePodcastRequest {
                ep_url: episode.get_episode_artwork(),
                episode_title: episode.get_episode_title(),
                user_id: user_id.unwrap(), // replace with the actual user ID
            };
            let server_name = server_name_copy; // replace with the actual server name
            let api_key = api_key_copy; // replace with the actual API key
            let future = async move {
                let _ = call_save_episode(&server_name.unwrap(), &api_key.flatten(), &request).await;
            };
            wasm_bindgen_futures::spawn_local(future);
            // dropdown_open.set(false);
        })
    };

    let download_api_key = api_key.clone();
    let download_server_name = server_name.clone();

    let on_download_episode = {
        let episode = props.episode.clone();
        Callback::from(move |_| {
            let server_name_copy = download_server_name.clone();
            let api_key_copy = download_api_key.clone();
            let request = DownloadEpisodeRequest {
                episode_id: episode.get_episode_id(),
                user_id: user_id.unwrap(), // replace with the actual user ID
            };
            let server_name = server_name_copy; // replace with the actual server name
            let api_key = api_key_copy; // replace with the actual API key
            let future = async move {
                let _ = call_download_episode(&server_name.unwrap(), &api_key.flatten(), &request).await;
            };
            wasm_bindgen_futures::spawn_local(future);
            // dropdown_open.set(false);
        })
    };

    html! {
        <>
        <div class="relative inline-block">
        <button
            id="dropdown-button"
            onclick={toggle_dropdown.clone()}
            class="item-container-button border-solid border selector-button font-bold py-2 px-4 rounded-full w-10 h-10 flex items-center justify-center"
        >
            <span class="material-icons">{"more_vert"}</span>
        </button>
        // Dropdown Content
        {
            if *dropdown_open {
                html! {
                    <div class="dropdown-content-class border border-solid absolute z-10 divide-y rounded-lg shadow w-48">
                        <ul class="dropdown-container py-2 text-sm text-gray-700">
                            <li class="dropdown-option" onclick={on_add_to_queue.clone()}>{ "Queue Episode" }</li>
                            <li class="dropdown-option" onclick={on_save_episode.clone()}>{ "Save Episode" }</li>
                            <li class="dropdown-option" onclick={on_download_episode.clone()}>{ "Download Episode" }</li>
                            // Add more categories as needed
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }
        }
        </div>
        </>
    }

}


pub fn empty_message(header: &str, paragraph: &str) -> Html {
    html! {
        <div class="empty-episodes-container">
            <img src="static/assets/favicon.png" alt="Logo" class="logo"/>
            <h1 class="page-paragraphs">{ header }</h1>
            <p class="page-paragraphs">{ paragraph }</p>
        </div>
    }
}

pub trait EpisodeTrait {
    fn get_episode_artwork(&self) -> String;
    fn get_episode_title(&self) -> String;
    fn get_episode_id(&self) -> i32;
    fn clone_box(&self) -> Box<dyn EpisodeTrait>;
    // fn eq(&self, other: &dyn EpisodeTrait) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for ContextButtonProps {
    fn eq(&self, other: &Self) -> bool {
        if let Some(other) = self.episode.as_any().downcast_ref::<Episode>() {
            if let Some(self_episode) = self.episode.as_any().downcast_ref::<Episode>() {
                return self_episode == other;
            }
        }

        if let Some(other) = self.episode.as_any().downcast_ref::<QueuedEpisode>() {
            if let Some(self_episode) = self.episode.as_any().downcast_ref::<QueuedEpisode>() {
                return self_episode == other;
            }
        }

        false
    }
}

impl Clone for Box<dyn EpisodeTrait> {
    fn clone(&self) -> Box<dyn EpisodeTrait> {
        self.clone_box()
    }
}

impl EpisodeTrait for Episode {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    // Implement other methods
}

impl EpisodeTrait for QueuedEpisode {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

impl EpisodeTrait for SavedEpisode {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EpisodeTrait for HistoryEpisode {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EpisodeTrait for EpisodeDownload {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EpisodeTrait for SearchEpisode {
    fn get_episode_artwork(&self) -> String {
        self.EpisodeArtwork.clone()
    }

    fn get_episode_title(&self) -> String {
        self.EpisodeTitle.clone()
    }

    fn get_episode_id(&self) -> i32 {
        self.EpisodeID.clone()
    }

    fn clone_box(&self) -> Box<dyn EpisodeTrait> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
    

    // Implement other methods

pub fn episode_item(
    episode: Box<dyn EpisodeTrait>,
    description: String,
    is_expanded: bool,
    format_release: &str,
    on_play_click: Callback<MouseEvent>,
    toggle_expanded: Callback<MouseEvent>,
) -> Html {
    html! {
        <div>
            <div class="item-container border-solid border flex items-center mb-4 shadow-md rounded-lg h-full">
                <img 
                    src={episode.get_episode_artwork()} 
                    alt={format!("Cover for {}", episode.get_episode_title())} 
                    class="w-2/12 md:w-4/12 object-cover pl-4"
                />
                <div class="flex flex-col p-4 space-y-2 flex-grow md:w-5/12">
                    <p class="item_container-text text-xl font-semibold">{ episode.get_episode_title() }</p>
                    <hr class="my-2 border-t hidden md:block"/>
                    {
                        html! {
                            <div class="item_container-text hidden md:block">
                                <div class="item_container-text episode-description-container">
                                    <SafeHtml html={description} />
                                </div>
                                <a class="link hover:underline cursor-pointer mt-4" onclick={toggle_expanded}>
                                    { if is_expanded { "See Less" } else { "See More" } }
                                </a>
                            </div>
                        }
                    }
                    <span class="episode-time-badge inline-flex items-center px-2.5 py-0.5 rounded me-2 border" style="flex-grow: 0; flex-shrink: 0; width: auto;">
                        <svg class="time-icon w-2.5 h-2.5 me-1.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M10 0a10 10 0 1 0 10 10A10.011 10.011 0 0 0 10 0Zm3.982 13.982a1 1 0 0 1-1.414 0l-3.274-3.274A1.012 1.012 0 0 1 9 10V6a1 1 0 0 1 2 0v3.586l2.982 2.982a1 1 0 0 1 0 1.414Z"/>
                        </svg>
                        { format_release }
                    </span>
                </div>
                <div class="flex flex-col md:flex-row space-y-6 md:space-y-0 md:space-x-6 items-center h-full w-2/12 md:w-2/12 px-2 md:px-4">
                    <button
                        class="item-container-button border-solid border selector-button font-bold py-2 px-4 rounded-full w-10 h-10 flex items-center justify-center"
                        onclick={on_play_click}
                    >
                        <span class="material-icons">{"play_arrow"}</span>
                    </button>
                    <ContextButton episode={episode.clone()} />
                </div>
            </div>
        </div>
    }
}