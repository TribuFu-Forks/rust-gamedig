//! Currently supported games.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Age of Chivalry
pub mod ageofchivalry;
/// Alien Swarm
pub mod alienswarm;
/// ARK: Survival Evolved
pub mod arkse;
/// ARMA 2: Operation Arrowhead
pub mod arma2oa;
/// Alien Swarm: Reactive Drop
pub mod asrd;
/// Avorion
pub mod avorion;
/// Ballistic Overkill
pub mod ballisticoverkill;
/// Battalion 1944
pub mod bat1944;
/// Battlefield 1942
pub mod bf1942;
/// Black Mesa
pub mod blackmesa;
/// BrainBread 2
pub mod brainbread2;
/// Codename CURE
pub mod codenamecure;
/// Colony Survival
pub mod colonysurvival;
/// Creativerse
pub mod creativerse;
/// Crysis Wars
pub mod crysiswars;
/// Counter-Strike
pub mod cs;
/// Counter Strike: Condition Zero
pub mod cscz;
/// Counter-Strike: Global Offensive
pub mod csgo;
/// Counter-Strike: Source
pub mod css;
/// Day of Defeat
pub mod dod;
/// Day of Defeat: Source
pub mod dods;
/// Day of Infamy
pub mod doi;
/// Don't Starve Together
pub mod dst;
/// Frontlines: Fuel of War
pub mod ffow;
/// Garry's Mod
pub mod garrysmod;
/// Halo: Combat Evolved
pub mod haloce;
/// Half-Life 2 Deathmatch
pub mod hl2dm;
/// Half-Life Deathmatch: Source
pub mod hldms;
/// Hell Let Loose
pub mod hll;
/// Insurgency
pub mod insurgency;
/// Insurgency: Modern Infantry Combat
pub mod insurgencymic;
/// Insurgency: Sandstorm
pub mod insurgencysandstorm;
/// Just Cause 2: Multiplayer
pub mod jc2mp;
/// Left 4 Dead
pub mod left4dead;
/// Left 4 Dead 2
pub mod left4dead2;
/// Minecraft
pub mod mc;
/// Operation: Harsh Doorstop
pub mod ohd;
/// Onset
pub mod onset;
/// Project Zomboid
pub mod przomboid;
/// Quake 1
pub mod quake1;
/// Quake 2
pub mod quake2;
/// Quake 3: Arena
pub mod quake3;
/// Risk of Rain 2
pub mod ror2;
/// Rust
pub mod rust;
/// 7 Days To Die
pub mod sd2d;
/// The Ship
pub mod ship;
/// Soldier of Fortune 2
pub mod sof2;
/// Serious Sam
pub mod ss;
/// Sven Co-op
pub mod svencoop;
/// Team Fortress 2
pub mod tf2;
/// Team Fortress Classic
pub mod tfc;
/// The Forest
pub mod theforest;
/// Unturned
pub mod unturned;
/// Unreal Tournament
pub mod ut;
/// V Rising
pub mod vrising;
/// Warsow
pub mod warsow;

use crate::protocols::gamespy::GameSpyVersion;
use crate::protocols::quake::QuakeVersion;
use crate::protocols::types::{CommonResponse, ExtraRequestSettings, ProprietaryProtocol, TimeoutSettings};
use crate::protocols::{self, Protocol};
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

/// Definition of a game
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    /// Full name of the game
    pub name: &'static str,
    /// Default port used by game
    pub default_port: u16,
    /// The protocol the game's query uses
    pub protocol: Protocol,
}

#[cfg(feature = "game_defs")]
mod definitions;

#[cfg(feature = "game_defs")]
pub use definitions::GAMES;

/// Make a query given a game definition
#[inline]
pub fn query(game: &Game, address: &IpAddr, port: Option<u16>) -> GDResult<Box<dyn CommonResponse>> {
    query_with_timeout_and_extra_settings(game, address, port, None, None)
}

/// Make a query given a game definition and timeout settings
#[inline]
pub fn query_with_timeout(
    game: &Game,
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    query_with_timeout_and_extra_settings(game, address, port, timeout_settings, None)
}

/// Make a query given a game definition, timeout settings, and extra settings
pub fn query_with_timeout_and_extra_settings(
    game: &Game,
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
    extra_settings: Option<ExtraRequestSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    let socket_addr = SocketAddr::new(*address, port.unwrap_or(game.default_port));
    Ok(match &game.protocol {
        Protocol::Valve(steam_app) => {
            protocols::valve::query(
                &socket_addr,
                steam_app.as_engine(),
                extra_settings.map(ExtraRequestSettings::into),
                timeout_settings,
            )
            .map(Box::new)?
        }
        Protocol::Minecraft(version) => {
            match version {
                Some(protocols::minecraft::Server::Java) => {
                    protocols::minecraft::query_java(
                        &socket_addr,
                        timeout_settings,
                        extra_settings.map(ExtraRequestSettings::into),
                    )
                    .map(Box::new)?
                }
                Some(protocols::minecraft::Server::Bedrock) => {
                    protocols::minecraft::query_bedrock(&socket_addr, timeout_settings).map(Box::new)?
                }
                Some(protocols::minecraft::Server::Legacy(group)) => {
                    protocols::minecraft::query_legacy_specific(*group, &socket_addr, timeout_settings).map(Box::new)?
                }
                None => {
                    protocols::minecraft::query(
                        &socket_addr,
                        timeout_settings,
                        extra_settings.map(ExtraRequestSettings::into),
                    )
                    .map(Box::new)?
                }
            }
        }
        Protocol::Gamespy(version) => {
            match version {
                GameSpyVersion::One => protocols::gamespy::one::query(&socket_addr, timeout_settings).map(Box::new)?,
                GameSpyVersion::Two => protocols::gamespy::two::query(&socket_addr, timeout_settings).map(Box::new)?,
                GameSpyVersion::Three => {
                    protocols::gamespy::three::query(&socket_addr, timeout_settings).map(Box::new)?
                }
            }
        }
        Protocol::Quake(version) => {
            match version {
                QuakeVersion::One => protocols::quake::one::query(&socket_addr, timeout_settings).map(Box::new)?,
                QuakeVersion::Two => protocols::quake::two::query(&socket_addr, timeout_settings).map(Box::new)?,
                QuakeVersion::Three => protocols::quake::three::query(&socket_addr, timeout_settings).map(Box::new)?,
            }
        }
        Protocol::PROPRIETARY(protocol) => {
            match protocol {
                ProprietaryProtocol::TheShip => {
                    ship::query_with_timeout(address, port, timeout_settings).map(Box::new)?
                }
                ProprietaryProtocol::FFOW => ffow::query_with_timeout(address, port, timeout_settings).map(Box::new)?,
                ProprietaryProtocol::JC2MP => {
                    jc2mp::query_with_timeout(address, port, timeout_settings).map(Box::new)?
                }
            }
        }
    })
}
