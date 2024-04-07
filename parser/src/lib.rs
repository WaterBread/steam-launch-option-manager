mod config_parse;
mod local_config_parse;

pub use config_parse::{parse, serialize};
pub use local_config_parse::{parse as local_parse, serialize as local_serialize};
