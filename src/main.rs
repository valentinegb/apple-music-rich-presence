use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use apple_music::{AppleMusic, ApplicationData, PlayerState, Track};
use discord_rich_presence::{
    activity::{Activity, ActivityType, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

fn update_rich_presence(
    client: &mut DiscordIpcClient,
    application_data: ApplicationData,
    mut track: Track,
    paused: bool,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Updating rich presence to {} by {}{}",
        track.name,
        track.artist,
        if paused { ", paused" } else { "" },
    );

    let artwork_url = track
        .artwork_url()
        .clone()
        .ok_or("Track has no album artwork")?;
    let track_url = track.track_url().clone().ok_or("Track has no URL")?;
    let mut assets = Assets::new()
        .large_image(&artwork_url)
        .large_text(&track.album);

    if paused {
        assets = assets.small_image("paused").small_text("Paused");
    }

    let mut payload = Activity::new()
        .activity_type(ActivityType::Listening)
        .assets(assets)
        .details(&track.name)
        .state(&track.artist)
        .buttons(vec![Button::new("Open in Apple Music", &track_url)]);

    if !paused {
        payload = payload.timestamps(
            Timestamps::new().end(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64
                    + (track.finish
                        - application_data
                            .player_position
                            .ok_or("No player position")?)
                    .round() as i64,
            ),
        );
    }

    client.set_activity(payload)?;

    Ok(())
}

fn main() {
    // TODO: use `sysinfo` crate, idle while Discord isn't open

    let mut client = DiscordIpcClient::new("1273805325954187386").unwrap();

    client.connect().unwrap();
    println!("Connected to Discord");

    let mut last_track_id = None;
    let mut was_paused = false;

    loop {
        let application_data = match AppleMusic::get_application_data() {
            Ok(track) => track,
            Err(err) => {
                eprintln!("Failed to get application data: {err:#?}");

                continue;
            }
        };
        let track = match AppleMusic::get_current_track() {
            Ok(track) => track,
            Err(err) => {
                eprintln!("Failed to get current track: {err:#?}");

                continue;
            }
        };
        let paused = match application_data
            .player_state
            .as_ref()
            .unwrap_or(&PlayerState::Paused)
        {
            PlayerState::Paused => true,
            _ => false,
        };

        if let Some(last_track_id) = last_track_id {
            if track.id == last_track_id && paused == was_paused {
                continue;
            }
        }

        last_track_id = Some(track.id);
        was_paused = paused;

        if let Err(err) = update_rich_presence(&mut client, application_data, track, paused) {
            eprintln!("Failed to update rich presence: {err:#?}");
        }
    }
}
