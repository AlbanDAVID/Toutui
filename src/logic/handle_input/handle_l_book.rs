use crate::player::vlc::start_vlc::*;
use crate::player::vlc::fetch_vlc_data::*;
use crate::api::me::update_media_progress::*;
use crate::api::library_items::play_lib_item_or_pod::*;
use crate::api::sessions::sync_open_session::*;
use crate::api::sessions::close_open_session::*;


pub async fn handle_l_book(
  token: Option<&String>,
    ids_library_items: Vec<String>,
    selected: Option<usize>,
    port: String,
) {
    if let Some(index) = selected {
        if let Some(id) = ids_library_items.get(index) {
            if let Some(token) = token {
                if let Ok(info_item) = post_start_playback_session_book(Some(&token), id).await {
                    // clone otherwise, these variable will  be consumed and not available anymore
                    // for use outside start_vlc spawn
                    let token_clone = token.clone();
                    let port_clone = port.clone();
                    let info_item_clone = info_item.clone() ;
                    // start_vlc is launched in a spawn to allow fetch_vlc_data to start at the same time
                    tokio::spawn(async move {
                        start_vlc(&info_item_clone[0], &port_clone, &info_item_clone[1], Some(&token_clone)).await;
                    });

                    // Important, sleep time to 1s otherwise connection to vlc player will not have time to connect
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                    loop {
                        match fetch_vlc_data(port.clone()).await {
                            Ok(Some(data_fetched_from_vlc)) => {
                                //println!("Fetched data: {}", data_fetched.to_string());

                                // Important, sleep time to 1s otherwise connection to vlc player will not have time to connect
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                match fetch_vlc_is_playing(port.clone()).await {
                                    Ok(true) => {
                                        // the first datra fetched is sometimes 0 secondes, so we
                                        // want to be sure no send 0 secondes
                                        if Some(data_fetched_from_vlc) != Some(0) {
                                        let _ = sync_session(Some(&token), &info_item[3],Some(data_fetched_from_vlc), 1).await;
                                        let _ = update_media_progress_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2]).await;

                                        }
                                    },
                                    // `Ok(false)` means that the track is stopped but VLC still
                                    // open. Allow to track when the audio reached the end. And
                                    // differ from the case where the user just close VLC
                                    // during a playing (in this case we don't want to mark the
                                    // track as finished)
                                    Ok(false) => {
                                        let is_finised = true;
                                        let _ =  close_session(Some(&token), &info_item[3], Some(data_fetched_from_vlc), 1).await;
                                        let _ = update_media_progress2_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2], is_finised).await;
                                        break; 
                                    },
                                    // `Err` means :  VLC is close (because if VLC is not playing
                                    // anymore an error is send by `fetch_vlc_is_playing`).
                                    // The track is not finished. VLC is just stopped by the user.
                                    // Differ from the case above where the track reched the end.
                                    // can be buggy if we want to make sure that VLC is closed. not really reliable sometimes.
                                    Err(_) => {
                                        //TODO minor bug : be sure to close the session above
                                        // close session when VLC is quitted
                                        let _ =  close_session(Some(&token), &info_item[3], Some(data_fetched_from_vlc), 1).await;
                                        // send one last time media progress (bug to retrieve media
                                        // progress otherwise)
                                        let _ = update_media_progress_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2]).await;
                                        //eprintln!("Error fetching play status: {}", e);
                                        break; 
                                    }
                                }

                            }
                            Ok(None) => {
                                break; // Exit if no data available
                            }
                            Err(_e) => {
                                break; // Exit on error
                            }
                        }
                    }
                } else {

                    eprintln!("Failed to start playback session");
                }
            }
        }
    }
}
