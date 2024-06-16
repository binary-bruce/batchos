use log::Log;

const COLOR_RED: u8 = 31;
const COLOR_BRIGHT_YELLOW: u8 = 93;
const COLOR_BLUE: u8 = 34;
const COLOR_GREEN: u8 = 32;
const COLOR_BRIGHT_BLACK: u8 = 90;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            log::Level::Error => COLOR_RED,
            log::Level::Warn => COLOR_BRIGHT_YELLOW,
            log::Level::Info => COLOR_BLUE,
            log::Level::Debug => COLOR_GREEN,
            log::Level::Trace => COLOR_BRIGHT_BLACK,
        };

        println!(
            "\u{1B}[{}m{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => log::LevelFilter::Error,
        Some("WARN") => log::LevelFilter::Warn,
        Some("INFO") => log::LevelFilter::Info,
        Some("DEBUG") => log::LevelFilter::Debug,
        Some("TRACE") => log::LevelFilter::Trace,
        Some(&_) => log::LevelFilter::Trace,
        None => log::LevelFilter::Off,
    });
}
