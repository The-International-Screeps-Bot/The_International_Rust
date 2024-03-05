use core::panic::PanicInfo;
use js_sys::JsString;
use log::{error, LevelFilter, Log};
use std::{fmt::Write, panic};
use web_sys::console as js_console;

const TRACE_COLOR: &str = "#999999";
const DEBUG_COLOR: &str = "#008c96";
const INFO_COLOR: &str = "#dddddd";
const WARN_COLOR: &str = "#f2d99a";
const WARN_BG_COLOR: &str = "#42381f";
const ERR_COLOR: &str = "#cf90a8";
const ERR_BG_COLOR: &str = "#4b2f36";

struct JsLogger;

impl Log for JsLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // the log methods should never be used to output HTML, so escape everything here
            let unescaped = record.args().to_string();
            let sanitized = html_escape::encode_safe(unescaped.as_str());
            match record.level() {
                log::Level::Trace => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{TRACE_COLOR}\">[TRACE] {}</span>",
                    sanitized
                ))),
                log::Level::Debug => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{DEBUG_COLOR}\">[DEBUG] {}</style>",
                    sanitized
                ))),
                log::Level::Info => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{INFO_COLOR}\">[INFO] {}",
                    sanitized
                ))),
                log::Level::Warn => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{WARN_COLOR};background-color:{WARN_BG_COLOR}\">⚠️ [WARN] {}</span>",
                    sanitized
                ))),
                log::Level::Error => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{ERR_COLOR};background-color:{ERR_BG_COLOR}\">⛔ [ERROR] {}</span>",
                    sanitized
                ))),
            }
        }
    }

    fn flush(&self) {}
}

pub fn setup_logger(level: LevelFilter) {
    log::set_logger(&JsLogger).unwrap();
    log::set_max_level(level);

    panic::set_hook(Box::new(panic_hook));
}

fn panic_hook(info: &PanicInfo) {
    // import Error to get backtrace info (backtraces don't work in wasm)
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    extern "C" {
        type Error;

        #[wasm_bindgen(constructor)]
        fn new() -> Error;

        #[wasm_bindgen(structural, method, getter)]
        fn stack(error: &Error) -> String;

        #[wasm_bindgen(static_method_of = Error, setter, js_name = stackTraceLimit)]
        fn stack_trace_limit(size: f32);
    }

    let mut fmt_error = String::new();
    let _ = writeln!(fmt_error, "{}", info);

    // this could be controlled with an env var at compilation instead
    const SHOW_BACKTRACE: bool = true;

    if SHOW_BACKTRACE {
        Error::stack_trace_limit(10000_f32);
        let stack = Error::new().stack();
        // Skip all frames before the special symbol `__rust_end_short_backtrace`
        // and then skip that frame too.
        // Note: sometimes wasm-opt seems to delete that symbol.
        if stack.contains("__rust_end_short_backtrace") {
            for line in stack
                .lines()
                .skip_while(|line| !line.contains("__rust_end_short_backtrace"))
                .skip(1)
            {
                let _ = writeln!(fmt_error, "{}", line);
            }
        } else {
            // If there was no `__rust_end_short_backtrace` symbol, use the whole stack
            // but skip the first line, it just says Error.
            let (_, stack) = stack.split_once("\n").unwrap();
            let _ = writeln!(fmt_error, "{}", stack);
        }
    }

    error!("{}", fmt_error);
}
