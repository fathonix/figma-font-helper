extern crate flexi_logger;

use flexi_logger::{opt_format, Logger};

pub fn init() {
  Logger::with_env_or_str(
    "font_helper=debug, finder=debug, libfonthelper=debug, simple_server=info",
  )
  .log_to_file()
  .directory(dirs::config_dir().unwrap().join("figma-font-helper/log").display().to_string())
  .format(opt_format)
  .start()
  .unwrap();
}
