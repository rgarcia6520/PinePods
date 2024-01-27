use yew::{function_component, html, use_node_ref, Html, Properties, Callback, MouseEvent};
use yew::prelude::*;
use super::app_drawer::App_drawer;
use super::gen_components::{Search_nav, empty_message, episode_item};
use crate::requests::pod_req;
use crate::requests::search_pods::{call_search_database, SearchRequest, SearchResponse};
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState};
use crate::components::audio::AudioPlayer;
use crate::components::gen_funcs::{sanitize_html_with_blank_target, truncate_description};
use crate::components::audio::on_play_click;
use crate::components::episodes_layout::AppStateMsg;
use crate::components::gen_funcs::check_auth;
use web_sys::HtmlInputElement;
use web_sys::HtmlElement;
use wasm_bindgen_futures::spawn_local;
use async_std::task::sleep;
use std::time::Duration;
use web_sys::window;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::closure::Closure;

#[derive(Properties, Clone, PartialEq)]
pub struct SearchProps {
    pub on_search: Callback<String>,
}

#[function_component(Search)]
pub fn search(props: &SearchProps) -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();
    let search_dispatch = dispatch.clone();

    check_auth(effect_dispatch);

    // let error = use_state(|| None);
    let (post_state, post_dispatch) = use_store::<AppState>();
    let (audio_state, audio_dispatch) = use_store::<UIState>();
    let dropdown_open = use_state(|| false);
    // let search_results = use_state(|| Vec::new());
    // let search_results_clone = search_results.clone();


    let input_ref = use_node_ref();
    let input_ref_clone1 = input_ref.clone();
    let input_ref_clone2 = input_ref.clone();
    let form_ref = NodeRef::default();
    let form_ref_clone1 = form_ref.clone();
    let form_ref_clone2 = form_ref.clone();
    let container_ref = use_node_ref();
    let container_ref_clone1 = container_ref.clone();
    let on_search = props.on_search.clone();

    let api_key = post_state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let user_id = post_state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let server_name = post_state.auth_details.as_ref().map(|ud| ud.server_name.clone());

    // let on_click = Callback::from(move |_| {
    //     if let Some(form) = input_ref_clone1.cast::<HtmlElement>() {
    //         form.class_list().add_1("move-to-top").unwrap();
    //     }
    // });

    let api_key_submit = api_key.clone();
    let user_id_submit = user_id.clone();
    let server_name_submit = server_name.clone();


    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        event.prevent_default();
        let container_ref_submit_clone1 = container_ref_clone1.clone();

        if let Some(form) = form_ref_clone1.cast::<HtmlElement>() {
            form.class_list().add_1("move-to-top").unwrap();
        }

        if let Some(form) = input_ref_clone1.cast::<HtmlElement>() {
            form.class_list().add_1("move-to-top").unwrap();
        }
        web_sys::console::log_1(&format!("server_name_submit: {:?}", server_name_submit).into());
        web_sys::console::log_1(&format!("api_key_submit: {:?}", api_key_submit).into());
        web_sys::console::log_1(&format!("user_id_submit: {:?}", user_id_submit).into());

        // Clone the necessary variables
        let server_name_submit = server_name_submit.clone();
        let api_key_submit = api_key_submit.clone();
        let user_id_submit = user_id_submit.clone();
        // let search_results = search_results_clone.clone();
        let mut search_request = None;
        web_sys::console::log_1(&"Before some statement".into());
        if let Some(input_element) = input_ref_clone2.cast::<HtmlInputElement>() {
            let search_term = input_element.value();
            web_sys::console::log_1(&"Inside some".into());
            search_request = Some(SearchRequest {
                search_term,
                user_id: user_id_submit.unwrap(), // replace with the actual user id
            });
        } else {
            web_sys::console::log_1(&"input_ref_clone2 is not an HtmlInputElement".into());
        }
        let future_dispatch = search_dispatch.clone();
        let future = async move {
            sleep(Duration::from_secs(1)).await;
            if let Some(container) = container_ref_submit_clone1.cast::<HtmlElement>() {
                container.class_list().add_1("shrink-input").unwrap();
            }
            if let Some(search_request) = search_request {
                web_sys::console::log_1(&format!("server_name: {:?}", server_name_submit).into());
                web_sys::console::log_1(&format!("api_key: {:?}", api_key_submit).into());
                web_sys::console::log_1(&format!("search_request: {:?}", search_request).into());
                let dispatch = future_dispatch.clone();
                match call_search_database(&server_name_submit.unwrap(), &api_key_submit.flatten(), &search_request).await {
                    Ok(results) => {
                        dispatch.reduce_mut(move |state| {
                            state.search_episodes = Some(SearchResponse { data: results });
                        });
                        // Update the search results state
                        // search_results.set(results);
                    }
                    Err(e) => {
                        // Handle the error
                        web_sys::console::log_1(&format!("Failed to search database: {:?}", e).into()); // Log for debugging
                    }
                }
            }
        };
        spawn_local(future);
    });

    

    html! {
        <>
        <div class="search-page-container">
            <Search_nav />
            <div class="search-container" ref={container_ref.clone()}>
                <form class="search-page-input" onsubmit={on_submit} ref={form_ref.clone()}>
                    <label for="search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">{ "Search" }</label>
                    <div class="relative">
                        <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                            <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
                            </svg>
                        </div>
                        <input type="search" id="search" class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search for a podcast, episode, or description" ref={input_ref.clone()}/>
                        <button type="submit" class="text-white absolute end-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Search" }</button>
                    </div>
                </form>
            </div>
            {
                if let Some(search_eps) = state.search_episodes.clone() {
                    let int_search_eps = search_eps.clone();
                    let episodes = int_search_eps.data;
                    if episodes.is_empty() {
                                // Render "No Recent Episodes Found" if episodes list is empty
                                empty_message(
                                    "No Search Results Found",
                                    "Perhaps try again, but search for something slightly different :/"
                                )
                            } else {
                                episodes.into_iter().map(|episode| {
                                    let state_ep = state.clone();
                                    let id_string = &episode.EpisodeID.to_string();

                                    let is_expanded = state.expanded_descriptions.contains(id_string);

                                    let dispatch = dispatch.clone();

                                    let episode_url_clone = episode.EpisodeURL.clone();
                                    let episode_title_clone = episode.EpisodeTitle.clone();
                                    let episode_artwork_clone = episode.EpisodeArtwork.clone();
                                    let episode_duration_clone = episode.EpisodeDuration.clone();

                                    let sanitized_description = sanitize_html_with_blank_target(&episode.EpisodeDescription.clone());

                                    let (description, is_truncated) = if is_expanded {
                                        (sanitized_description, false)
                                    } else {
                                        truncate_description(sanitized_description, 300)
                                    };

                                    let toggle_expanded = {
                                        let search_dispatch_clone = dispatch.clone();
                                        let state_clone = state.clone();
                                        let episode_guid = episode.EpisodeID.clone();

                                        Callback::from(move |_: MouseEvent| {
                                            let guid_clone = episode_guid.to_string().clone();
                                            let search_dispatch_call = search_dispatch_clone.clone();

                                            if state_clone.expanded_descriptions.contains(&guid_clone) {
                                                search_dispatch_call.apply(AppStateMsg::CollapseEpisode(guid_clone));
                                            } else {
                                                search_dispatch_call.apply(AppStateMsg::ExpandEpisode(guid_clone));
                                            }
                                        })
                                    };

                                    let episode_url_for_closure = episode_url_clone.clone();
                                    let episode_title_for_closure = episode_title_clone.clone();
                                    let episode_artwork_for_closure = episode_artwork_clone.clone();
                                    let episode_duration_for_closure = episode_duration_clone.clone();
                                    let audio_dispatch = audio_dispatch.clone();
                                    let play_state = state_ep.clone();

                                    let on_play_click = on_play_click(
                                        episode_url_for_closure.clone(),
                                        episode_title_for_closure.clone(),
                                        episode_artwork_for_closure.clone(),
                                        episode_duration_for_closure.clone(),
                                        audio_dispatch.clone(),
                                    );

                                    let format_release = format!("Released on: {}", &episode.EpisodePubDate);
                                    let item = episode_item(
                                        Box::new(episode),
                                        description.clone(),
                                        is_expanded,
                                        &format_release,
                                        on_play_click,
                                        toggle_expanded,
                                    );

                                    item
                                }).collect::<Html>()
                            }
                    // } else {
                    //     empty_message(
                    //         "No Recent Episodes Found",
                    //         "You can add new podcasts by using the search bar above. Search for your favorite podcast and click the plus button to add it."
                    //     )
                    // }
                } else {
                    html! {} 
                }
            }
            <App_drawer />
            {
                if let Some(audio_props) = &audio_state.currently_playing {
                    html! { <AudioPlayer src={audio_props.src.clone()} title={audio_props.title.clone()} artwork_url={audio_props.artwork_url.clone()} duration={audio_props.duration.clone()} duration_sec={audio_props.duration_sec.clone()} /> }
                } else {
                    html! {}
                }
            }
        </div>
        </>
    }
}
