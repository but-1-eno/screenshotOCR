mod capture;
mod config;
mod hotkey;
mod ocr;

use anyhow::{Context, Result};
use global_hotkey::GlobalHotKeyEvent;
use std::{env, path::PathBuf, thread, time::Duration};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("ocr") => recognize_file(args.next().context("usage: screenshot_ocr ocr <image>")?),
        Some("capture") => capture_region(args.collect()),
        Some("daemon") | None => run_daemon(),
        Some(command) => anyhow::bail!("unknown command: {command}"),
    }
}

fn recognize_file(path: String) -> Result<()> {
    let image = image::open(&path).with_context(|| format!("unable to open {path}"))?;
    println!("{}", ocr::recognize(&image)?);
    Ok(())
}

fn capture_region(args: Vec<String>) -> Result<()> {
    if args.len() != 4 {
        anyhow::bail!("usage: screenshot_ocr capture <x> <y> <width> <height>");
    }
    let values = args
        .iter()
        .map(|value| value.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let image = capture::capture_region(values[0], values[1], values[2] as u32, values[3] as u32)?;
    let path = PathBuf::from("screenshot.png");
    image.save(&path)?;
    println!("Saved {}", path.display());
    println!("{}", ocr::recognize(&image)?);
    Ok(())
}

fn run_daemon() -> Result<()> {
    let config = config::AppConfig::load_or_prompt()?;
    let service = hotkey::HotkeyService::register(&config.hotkey)
        .with_context(|| format!("unable to register {}", config.hotkey))?;
    println!("Listening for {}. Press Ctrl+C to stop.", config.hotkey);
    loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if service.is_triggered(&event) {
                println!("Hotkey activated; selection overlay will be connected here.");
            }
        }
        thread::sleep(Duration::from_millis(25));
    }
}
