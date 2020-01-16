/*!lib.rs
 * Utils lib.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#[macro_use]
extern crate lazy_static;
pub extern crate shared_memory;

pub mod gpioregs;
pub mod log;

use gpioregs::*;
use shared_memory::*;

static GLOBAL_LOCK_ID: usize = 0;

pub fn init_shared_memory() -> Result<SharedMem, SharedMemError> {
    let appdata_path = std::env::var("APPDATA").unwrap() + "\\simpi";
    let link_path = appdata_path + "\\simpi.link";
    println!("Attempting to create/open custom gpioregs !");
    let mut gpioregs = match SharedMem::create_linked(link_path.clone(), LockType::Mutex, 24) {
        // We created and own this mapping
        Ok(v) => v,
        // Link file already exists
        Err(SharedMemError::LinkExists) =>
            SharedMem::open_linked(link_path.clone())?,
        Err(e) => return Err(e),
    };

    println!("Mapping info : {}", gpioregs);

    if gpioregs.num_locks() != 1 {
        println!("Expected to only have 1 lock in shared mapping !");
        return Err(SharedMemError::InvalidHeader);
    } else {
        if gpioregs.is_owner() {
            let mut gpioregs = gpioregs.wlock::<RegMemory>(GLOBAL_LOCK_ID)?;
            gpioregs.reset();
            println!("Yay we are owner!!");
        } else {
            println!("Somebody was faster than us:/");
        }
    }
    Ok(gpioregs)
}
