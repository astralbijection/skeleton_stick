use clap::Parser;

pub mod cli;
pub mod hid;

pub mod driver;
pub mod ui;

fn main() {
    let args = cli::Args::parse();

    match args.driver {
        cli::Driver::Simulator => {
            #[cfg(feature = "simulator")]
            driver::simulator::run();

            #[cfg(not(feature = "simulator"))]
            panic!("feature `simulator` was not compiled into this binary!")
        }
        cli::Driver::PiZeroWaveshareOLED => todo!(),
    }
}
