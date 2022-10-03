extern crate flexi_logger;

use flexi_logger::{opt_format, Logger};

pub fn init() {
  Logger::with_env_or_str(
    "font_helper=debug, finder=debug, libfonthelper=debug, simple_server=info",
  )
  .log_to_file()
  .directory(
    dirs::config_dir()
      .expect("Unable to retrieve the config directory")
      .join("figma-font-helper/log"))
  .format(opt_format)
  .start()
  .unwrap();
}
