#![no_std]

#[macro_use]
extern crate axlog;

mod lang_items;

const LOGO: &str = r#"
       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"
"#;

extern "Rust" {
    fn main();
}

#[no_mangle]
pub fn rust_main() -> ! {
    println!("{}", LOGO);
    println!(
        "\
        arch = {}\n\
        platform = {}\n\
        build_mode = {}\n\
        log_level = {}\n\
        ",
        option_env!("ARCH").unwrap_or(""),
        option_env!("PLATFORM").unwrap_or(""),
        option_env!("MODE").unwrap_or(""),
        option_env!("LOG").unwrap_or(""),
    );

    axlog::init();
    axlog::set_max_level(option_env!("LOG").unwrap_or(""));
    info!("Logging is enabled.");

    unsafe { main() };

    axhal::misc::terminate()
}