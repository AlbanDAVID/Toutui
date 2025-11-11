use std::io::{self, Write};
use std::net::TcpStream;
use crate::db::crud::*;
use std::thread;
use std::time::Duration;

pub fn handle_key_player(key: &str, address: &str, port: &str, is_playback: &mut bool, username: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    let jump = "10";

    match key {
        // toggle playback/pause
        " " => {
            match get_listening_session() {
                Ok(Some(session)) => {
                    if session.is_playback {
                    let _ = update_is_playback("0", session.id_session.as_str());
                    } else {
                    let _ = update_is_playback("1", session.id_session.as_str());
                    }
                }
                Ok(None) => {

                }
                Err(_e) => {

                }
            }
            if *is_playback {
                writeln!(stream, "pause")?;
            } else {
                writeln!(stream, "play")?;
            }
            *is_playback = !*is_playback;
        }

        // For some cmd, below, need pause => cmd => play
        // Allow vlc buffer issue. 
        // Futhermore, need a thread for macos otherwise vlc buffer issue
        // Otherwise buffer issue and the player freeze
        // But maybe it's not necessary because I test toutui on macos with a VM
        // and maybe the VM add a little delay.. but for now I try like this

        // jump forward
        "p" => {
            writeln!(stream, "pause")?; 
            writeln!(stream, "seek +{}", jump)?;
            if cfg!(target_os = "macos") {
            thread::sleep(Duration::from_millis(500));
            }
            writeln!(stream, "play")?;
        }
        // jump backward
        "u" => {
            writeln!(stream, "pause")?;
            writeln!(stream, "seek -{}", jump)?;
            if cfg!(target_os = "macos") {
            thread::sleep(Duration::from_millis(500));
            }
            writeln!(stream, "play")?;
        }
        // next chapter
        "P" => {
            writeln!(stream, "pause")?;
            writeln!(stream, "next")?;
            if cfg!(target_os = "macos") {
            thread::sleep(Duration::from_millis(500));
            }
            writeln!(stream, "play")?;
        }
        // previous chapter
        "U" => {
            writeln!(stream, "pause")?;
            writeln!(stream, "prev")?;
            if cfg!(target_os = "macos") {
            thread::sleep(Duration::from_millis(500));
            }
            writeln!(stream, "play")?;
        }
        // volume up
        "o" => {
            writeln!(stream, "volup")?;
        }
        // volume down
        "i" => {
            writeln!(stream, "voldown")?;
        }
        // speed rate up
        "O" => {
            let _ = update_speed_rate(username, true);
            let speed_rate = get_speed_rate(username);
            writeln!(stream, "rate {}", speed_rate)?;
        }
        // speed rate down
        "I" => {
            let _ = update_speed_rate(username, false);
            let speed_rate = get_speed_rate(username);
            writeln!(stream, "rate {}", speed_rate)?;
        }
        // shutdown
        "Y" => {
            writeln!(stream, "shutdown")?;
        }
        _ => {}
    }

    Ok(())
}

