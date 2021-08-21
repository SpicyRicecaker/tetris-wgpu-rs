mod tetris_input;
mod tetromino;
pub mod config;
pub mod prod;

/// Universe is where all the functionality is
pub mod universe;

use std::collections::HashMap;

use tetromino::*;
use config::Config;

use tetris_input::*;
