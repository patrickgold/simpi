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

use tui::layout::Rect;
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

fn helper_is_rect_in_range(parent: Rect, child: Rect) -> bool {
    let p_x_min = parent.x;
    let p_x_max = parent.x + parent.width;
    let p_y_min = parent.y;
    let p_y_max = parent.y + parent.height;
    let c_x_min = child.x;
    let c_x_max = child.x + child.width;
    let c_y_min = child.y;
    let c_y_max = child.y + child.height;
    return c_x_min > p_x_min
        && c_y_min > p_y_min
        && c_x_max < p_x_max
        && c_y_max < p_y_max;
}
