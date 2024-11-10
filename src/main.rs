use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?; // Rawモードを有効化
    let mut stdout = io::stdout();

    // キーボード拡張フラグをプッシュ
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        )
    )?;

    println!("Cmd + Enter を押すとイベントが発生します。終了するには 'q' を押してください。");

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                println!("検出されたキーイベント: {:?}", key_event);
                io::stdout().flush().unwrap();
                if key_event.code == KeyCode::Enter && key_event.modifiers.contains(KeyModifiers::META) {
                    println!("Cmd + Enter が押されました！");
                    io::stdout().flush().unwrap();
                }
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // キーボード拡張フラグをポップ
    execute!(stdout, PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;
    Ok(())
}
