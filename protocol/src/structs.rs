// TODO move this to another plugin because protocol library should be replaceable

use crate::structs::dimension_codec::{DimensionType, WorldgenBiome};
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}

#[derive(Clone, Debug)]
pub struct Chat {
    // TODO
}

#[derive(Clone, Debug)]
pub struct Identifier {
    namespace: String,
    name: String,
}

impl Identifier {
    pub fn new<T, U>(namespace: T, name: U) -> Identifier
    where
        T: Into<String>,
        U: Into<String>,
    {
        Identifier {
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(self.namespace.clone() + ":" + &self.name))
    }
}

impl TryFrom<String> for Identifier {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let arr: Vec<&str> = value.split(":").collect();
        if let [namespace, name] = arr.as_slice() {
            Ok(Identifier::new(*namespace, *name))
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid identifier format",
            ))
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Serialize, Clone, Debug)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: DimensionType,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome: WorldgenBiome,
}

pub mod dimension_codec {
    use serde::Serialize;
    #[derive(Serialize, Clone, Debug)]
    pub struct DimensionType {
        pub r#type: String,
        pub value: Vec<Dimension>,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct Dimension {
        pub name: String,
        pub id: i32,
        pub element: DimensionElement,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct DimensionElement {
        pub piglin_safe: bool,
        pub natural: bool,
        pub ambient_light: f32,
        pub infiniburn: String,
        pub respawn_anchor_works: bool,
        pub has_skylight: bool,
        pub bed_works: bool,
        pub effects: String,
        pub has_raids: bool,
        pub height: i32,
        pub logical_height: i32,
        pub min_y: i32,
        pub coordinate_scale: f32,
        pub ultrawarm: bool,
        pub has_ceiling: bool,
        pub fixed_time: bool, // TODO Option<i32> but serialize as `false` if None
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct WorldgenBiome {
        pub r#type: String,
        pub value: Vec<Biome>,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct Biome {
        pub name: String,
        pub id: i32,
        pub element: BiomeElement,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct BiomeElement {
        pub precipitation: String,
        pub effects: Effects,
        pub depth: f32,
        pub temperature: f32,
        pub scale: f32,
        pub downfall: f32,
        pub category: String,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct Effects {
        pub sky_color: i32,
        pub water_fog_color: i32,
        pub fog_color: i32,
        pub water_color: i32,
        pub mood_sound: MoodSound,
    }
    #[derive(Serialize, Clone, Debug)]
    pub struct MoodSound {
        pub tick_delay: i32,
        pub offset: f32,
        pub sound: String,
        pub block_search_extent: i32,
    }
}

#[derive(Clone, Debug)]
pub enum Hand {
    Main,
    Off
}