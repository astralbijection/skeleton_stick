use clap::{arg, command, Parser, ValueEnum};

/// The Skeleton Stick device application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Which driver to use.
    #[arg(short, long, value_enum)]
    pub driver: Driver,
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Driver {
    Simulator,
    PiZeroWaveshareOLED,
}
