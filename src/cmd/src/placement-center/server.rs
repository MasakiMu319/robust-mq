use clap::Parser;
use common_base::{
    config::placement_center::{init_placement_center_conf_by_path, placement_center_conf},
    log::placement_center::init_placement_center_log,
};
use log::info;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";

#[derive(Debug, Parser)]
#[command(author="Maku319", version="0.0.1", about="RobustMQ: Next generation high-performance message queue.", long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}

fn main() {
    let args = ArgsParams::parse();
    init_placement_center_conf_by_path(&args.conf);
    init_placement_center_log();

    let conf = placement_center_conf();
    info!("{:?}", conf);

    info!("Server started.")
}
