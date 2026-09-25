use anyhow::{bail, Context, Result};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

pub struct HotkeyService {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl HotkeyService {
    pub fn register(value: &str) -> Result<Self> {
        let (modifiers, code) = parse(value)?;
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(Some(modifiers), code);
        manager.register(hotkey)?;
        Ok(Self { manager, hotkey })
    }

    pub fn is_triggered(&self, event: &GlobalHotKeyEvent) -> bool {
        event.id == self.hotkey.id()
    }

    pub fn unregister(self) -> Result<()> {
        self.manager.unregister(self.hotkey)?;
        Ok(())
    }
}

fn parse(value: &str) -> Result<(Modifiers, Code)> {
    let mut modifiers = Modifiers::empty();
    let mut key = None;
    for token in value
        .split('+')
        .map(|token| token.trim().to_ascii_uppercase())
    {
        match token.as_str() {
            "CTRL" | "CONTROL" => modifiers |= Modifiers::CONTROL,
            "SHIFT" => modifiers |= Modifiers::SHIFT,
            "ALT" => modifiers |= Modifiers::ALT,
            "WIN" | "META" => modifiers |= Modifiers::META,
            value if value.len() == 1 && value.as_bytes()[0].is_ascii_uppercase() => {
                key = Some(letter_code(value.as_bytes()[0]))
            }
            value if value.starts_with('F') => key = Some(function_code(value)?),
            _ => bail!("unsupported shortcut key: {token}"),
        }
    }
    Ok((modifiers, key.context("shortcut must include a key")?))
}

fn letter_code(value: u8) -> Code {
    match value {
        b'A' => Code::KeyA,
        b'B' => Code::KeyB,
        b'C' => Code::KeyC,
        b'D' => Code::KeyD,
        b'E' => Code::KeyE,
        b'F' => Code::KeyF,
        b'G' => Code::KeyG,
        b'H' => Code::KeyH,
        b'I' => Code::KeyI,
        b'J' => Code::KeyJ,
        b'K' => Code::KeyK,
        b'L' => Code::KeyL,
        b'M' => Code::KeyM,
        b'N' => Code::KeyN,
        b'O' => Code::KeyO,
        b'P' => Code::KeyP,
        b'Q' => Code::KeyQ,
        b'R' => Code::KeyR,
        b'S' => Code::KeyS,
        b'T' => Code::KeyT,
        b'U' => Code::KeyU,
        b'V' => Code::KeyV,
        b'W' => Code::KeyW,
        b'X' => Code::KeyX,
        b'Y' => Code::KeyY,
        _ => Code::KeyZ,
    }
}

fn function_code(value: &str) -> Result<Code> {
    let code = match value {
        "F1" => Code::F1,
        "F2" => Code::F2,
        "F3" => Code::F3,
        "F4" => Code::F4,
        "F5" => Code::F5,
        "F6" => Code::F6,
        "F7" => Code::F7,
        "F8" => Code::F8,
        "F9" => Code::F9,
        "F10" => Code::F10,
        "F11" => Code::F11,
        "F12" => Code::F12,
        _ => bail!("unsupported function key: {value}"),
    };
    Ok(code)
}
