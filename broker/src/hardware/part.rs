/*!part.rs
 * Hardware | Part definition.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

use super::button::Button;
use super::led::Led;

#[derive(Clone)]
pub enum Part {
    Button(Button),
    Led(Led),
}
