/*!mod.rs
 * Hardware lib. Provides support for unpacking simpi json files.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

pub mod board;
pub mod button;
pub mod led;
pub mod part;

pub use board::Board;
pub use button::Button;
pub use led::Led;
pub use part::Part;

use tui::style::Color;

fn helper_str_to_color(c: String) -> Result<Color, ()> {
    Ok(match c.to_lowercase().as_ref() {
        "reset"         => Color::Reset,
        "black"         => Color::Black,
        "red"           => Color::Red,
        "green"         => Color::Green,
        "yellow"        => Color::Yellow,
        "blue"          => Color::Blue,
        "magenta"       => Color::Magenta,
        "cyan"          => Color::Cyan,
        "gray"          => Color::Gray,
        "darkgray"      => Color::DarkGray,
        "lightred"      => Color::LightRed,
        "lightgreen"    => Color::LightGreen,
        "lightyellow"   => Color::LightYellow,
        "lightblue"     => Color::LightBlue,
        "lightmagenta"  => Color::LightMagenta,
        "lightcyan"     => Color::LightCyan,
        "white"         => Color::White,
        _ => return Err(()),
    })
}
