// Apple Music Rich Presence
// Copyright (C) 2024  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    env,
    error::Error,
    fs::File,
    process::ExitCode,
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use apple_music::{AppleMusic, PlayerState, Track};
use daemonize::Daemonize;
use discord_rich_presence::{
    activity::{Activity, ActivityType, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use log::{debug, error, info, trace, warn};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate};

const CLIENT_ID: &str = "1273805325954187386";
const EXPECT_CLIENT_INITIALIZED: &str = "Discord client has not been initialized";

fn inner_loop(
    sys: &mut sysinfo::System,
    discord_client_option: &mut Option<DiscordIpcClient>,
    last_track: &mut Option<Track>,
    was_paused: &mut bool,
) -> Result<(), Box<dyn Error>> {
    sys.refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new());

    let discord_is_open = sys
        .processes_by_exact_name("Discord".as_ref())
        .next()
        .is_some();
    let apple_music_is_open = sys
        .processes_by_exact_name("Music".as_ref())
        .next()
        .is_some();

    if discord_is_open {
        trace!("Discord is open");

        if apple_music_is_open {
            trace!("Apple Music is open");

            if discord_client_option.is_none() {
                let mut discord_client = DiscordIpcClient::new(CLIENT_ID)?;

                discord_client.connect()?;
                info!("Connected Discord client");

                *discord_client_option = Some(discord_client);
            }

            let app_data = AppleMusic::get_application_data();
            let is_paused = if let Ok(PlayerState::Playing) = app_data
                .as_ref()
                .map(|app| app.player_state.as_ref().unwrap_or(&PlayerState::Paused))
            {
                false
            } else {
                true
            };

            if let Ok(mut track) = AppleMusic::get_current_track() {
                trace!("Current track is something");

                if last_track
                    .as_ref()
                    .map(|last_track| last_track.id != track.id)
                    .unwrap_or(true)
                    || *was_paused != is_paused
                {
                    debug!("Something changed and activity needs to be updated");

                    let artwork_url = track.artwork_url().clone();
                    let track_url = track.track_url().clone();
                    let mut assets = Assets::new()
                        .large_image(artwork_url.as_ref().ok_or("Track does not have artwork")?)
                        .large_text(&track.album);

                    if is_paused {
                        assets = assets.small_image("paused").small_text("Paused");
                    }

                    let mut timestamps = Timestamps::new();

                    if !is_paused {
                        if let Ok(app_data) = app_data {
                            timestamps = Timestamps::new().end(
                                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64
                                    + (track.finish
                                        - app_data.player_position.ok_or("No player position")?)
                                    .round() as i64,
                            );
                        }
                    }

                    let buttons = if let Some(track_url) = track_url.as_ref() {
                        vec![Button::new("Open in Apple Music", track_url)]
                    } else {
                        warn!("Track has no URL");

                        vec![]
                    };

                    discord_client_option
                        .as_mut()
                        .ok_or(EXPECT_CLIENT_INITIALIZED)?
                        .set_activity(
                            Activity::new()
                                .activity_type(ActivityType::Listening)
                                .assets(assets)
                                .details(&track.name)
                                .state(&track.artist)
                                .timestamps(timestamps)
                                .buttons(buttons),
                        )?;
                    info!(
                        "{} by {} is {}, activity set",
                        track.name,
                        track.artist,
                        if is_paused { "paused" } else { "playing" },
                    );

                    *last_track = Some(track);
                }
            } else {
                trace!("There is no current track");

                if last_track.is_some() {
                    debug!("There was a track, now there is not");
                    discord_client_option
                        .as_mut()
                        .ok_or(EXPECT_CLIENT_INITIALIZED)?
                        .clear_activity()?;
                    info!("No song is playing, activity cleared");

                    *last_track = None;
                }
            }

            *was_paused = is_paused;
        } else {
            trace!("Apple Music is not open");

            if let Some(discord_client) = discord_client_option.as_mut() {
                discord_client.close()?;

                *discord_client_option = None;

                info!("Disconnected Discord client");
            }
        }
    } else {
        trace!("Discord is not open");

        if discord_client_option.is_some() {
            *discord_client_option = None;

            info!("Deinitialized Discord client");
        }
    }

    Ok(())
}

fn try_main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    debug!("Initialized logging");

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return Err("This system is not supported by sysinfo, a crucial dependency".into());
    }

    if env::var("DAEMON")
        .map(|daemon| daemon == "1")
        .unwrap_or(true)
    {
        let home = env::var("HOME")?;
        let stderr = File::create(format!("{home}/Library/Logs/apple-music-rich-presence.log",))?;

        Daemonize::new().stderr(stderr).start()?;
        debug!("Daemonized");
    }

    let min_iter_dur = Duration::from_secs(
        env::var("MIN_ITER_DUR")
            .unwrap_or("1".to_string())
            .parse()
            .map_err(|err| format!("Failed to parse `MIN_ITER_DUR`: {err}"))?,
    );
    let mut sys = sysinfo::System::new();
    let mut discord_client_option = None;
    let mut last_track = None;
    let mut was_paused = true;

    loop {
        let loop_start = Instant::now();

        if let Err(err) = inner_loop(
            &mut sys,
            &mut discord_client_option,
            &mut last_track,
            &mut was_paused,
        ) {
            warn!("{err}");
        }

        let loop_start_elapsed = loop_start.elapsed();

        if loop_start_elapsed < min_iter_dur {
            sleep(min_iter_dur - loop_start_elapsed);
        }
    }
}

fn main() -> ExitCode {
    if let Err(err) = try_main() {
        error!("{err}");

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS // Impossible to reach?
}
