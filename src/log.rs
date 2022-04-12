extern crate flexi_logger;
use flexi_logger::{opt_format, Logger};

pub fn init() {
    Logger::try_with_env_or_str(
        "font_helper=debug, finder=debug, libfonthelper=debug, simple_server=info",
    )
    .unwrap()
    .format(opt_format)
    .start()
    .unwrap();
}
