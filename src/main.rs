use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?; // Rawモードを有効化
    println!("Press Cmd + Enter to trigger the event. Press 'q' to exit.");

    loop {
        // イベントを待機
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                // Cmd (macOSの場合は`META`) + Enter キーが押されたかを判定
                if key_event.code == KeyCode::Enter && key_event.modifiers.contains(KeyModifiers::META) {
                    println!("Cmd + Enter key pressed!");
                    io::stdout().flush().unwrap();
                }
                // 'q'キーで終了
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
