use std::num::NonZeroUsize;

use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use strum::EnumString;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Display, EnumString, ValueEnum)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum CycleDirection {
    Previous,
    Next,
}

impl CycleDirection {
    #[must_use]
    pub const fn next_idx(&self, idx: usize, len: NonZeroUsize) -> usize {
        match self {
            Self::Previous => {
                if idx == 0 {
                    len.get() - 1
                } else {
                    idx - 1
                }
            }
            Self::Next => {
                if idx == len.get() - 1 {
                    0
                } else {
                    idx + 1
                }
            }
        }
    }
}
