use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use apple_music::{AppleMusic, Track};
use discord_rich_presence::{
    activity::{Activity, ActivityType, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

fn update_rich_presence(
    client: &mut DiscordIpcClient,
    mut track: Track,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Updating rich presence to {} by {}",
        track.name, track.artist,
    );

    let artwork_url = track
        .artwork_url()
        .clone()
        .ok_or("Track has no album artwork")?;
    let track_url = track.track_url().clone().ok_or("Track has no URL")?;
    let payload = Activity::new()
        .activity_type(ActivityType::Listening)
        .assets(
            Assets::new()
                .large_image(&artwork_url)
                .large_text(&track.album),
        )
        .details(&track.name)
        .state(&track.artist)
        .timestamps(
            Timestamps::new().end(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64
                    + (track.finish
                        - AppleMusic::get_application_data()?
                            .player_position
                            .ok_or("No player position")?)
                    .round() as i64,
            ),
        )
        .buttons(vec![Button::new("Open in Apple Music", &track_url)]);

    client.set_activity(payload)?;

    Ok(())
}

fn main() {
    let mut client = DiscordIpcClient::new("1273805325954187386").unwrap();

    client.connect().unwrap();

    let mut last_track_id = None;

    loop {
        let track = match AppleMusic::get_current_track() {
            Ok(track) => track,
            Err(err) => {
                eprintln!("Failed to get current track: {err:#?}");

                continue;
            }
        };

        if let Some(last_track_id) = last_track_id {
            if track.id == last_track_id {
                continue;
            }
        }

        last_track_id = Some(track.id);

        if let Err(err) = update_rich_presence(&mut client, track) {
            eprintln!("Failed to update rich presence: {err:#?}");
        }
    }
}
