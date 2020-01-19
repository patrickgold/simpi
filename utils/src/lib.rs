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
    // TODO: implement for Linux as well
    let appdata_path = std::env::var("APPDATA").unwrap() + "\\simpi";
    let link_path = appdata_path + "\\~simpi.link";
    log::info("Attempting to create/open shared gpioregs mapping...");
    let mut gpioregs = match SharedMem::create_linked(
        link_path.clone(), LockType::Mutex, std::mem::size_of::<RegMemory>()
    ) {
        // We created and own this mapping
        Ok(v) => v,
        // Link file already exists
        Err(SharedMemError::LinkExists) =>
            SharedMem::open_linked(link_path.clone())?,
        Err(e) => return Err(e),
    };

    log::info(format!("Mapping info : {}", gpioregs).as_ref());

    if gpioregs.num_locks() != 1 {
        log::error("Expected to only have 1 lock in shared mapping!");
        return Err(SharedMemError::InvalidHeader);
    } else {
        if gpioregs.is_owner() {
            let mut gpioregs = gpioregs.wlock::<RegMemory>(GLOBAL_LOCK_ID)?;
            gpioregs.reset();
            log::info("This process is owner of the shared mapping.");
        } else {
            log::info("This process is not owner of the shared mapping.");
        }
    }
    Ok(gpioregs)
}
