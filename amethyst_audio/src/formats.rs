use amethyst_assets::*;
use amethyst_error::Error;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AudioData(pub Vec<u8>);
amethyst_assets::register_format_type!(AudioData);

/// Loads audio from `.at3` files.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AT3Format;

amethyst_assets::register_format!("AT3", AT3Format as AudioData);
impl Format<AudioData> for AT3Format {
    fn name(&self) -> &'static str {
        "AT3"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<AudioData, Error> {
        Ok(AudioData(bytes))
    }
}
