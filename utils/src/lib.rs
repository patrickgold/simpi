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
use std::path::Path;

static GLOBAL_LOCK_ID: usize = 0;

pub struct ShMem {
    pub mem: SharedMem,
}
unsafe impl Send for ShMem {}
impl ShMem {
    pub fn new(mem: SharedMem) -> Self {
        Self { mem }
    }
    pub fn rlock<'a>(res: &'a Result<ShMem, SharedMemError>) -> ReadLockGuard<'_, RegMemory> {
        res.as_ref().unwrap().mem.rlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap()
    }
    pub fn wlock<'a>(res: &'a mut Result<ShMem, SharedMemError>) -> WriteLockGuard<'_, RegMemory> {
        res.as_mut().unwrap().mem.wlock::<RegMemory>(GLOBAL_LOCK_ID).unwrap()
    }
}

pub fn init_shared_memory() -> Result<ShMem, SharedMemError> {
    _init_shared_memory(0)
}

fn _init_shared_memory(n: usize) -> Result<ShMem, SharedMemError> {
    let sh_path: String;
    let win32_appdata = std::env::var("APPDATA").unwrap_or("#".to_owned());
    let linux_appdata = std::env::var("HOME").unwrap_or("#".to_owned());
    if Path::new(&win32_appdata).exists() {
        let simpi_dir = win32_appdata + "\\simpi";
        if !Path::new(&simpi_dir).exists() {
            match std::fs::create_dir(simpi_dir.clone()) {
                Ok(_) => {},
                Err(_) => return Err(SharedMemError::LinkDoesNotExist)
            }
        }
        sh_path = simpi_dir + "\\~simpi.link";
    } else if Path::new(&linux_appdata).exists() {
        let simpi_dir = linux_appdata + "/simpi";
        if !Path::new(&simpi_dir).exists() {
            match std::fs::create_dir(simpi_dir.clone()) {
                Ok(_) => {},
                Err(_) => return Err(SharedMemError::LinkDoesNotExist)
            }
        }
        sh_path = simpi_dir + "/~simpi.link";
    } else {
        return Err(SharedMemError::LinkDoesNotExist);
    }
    log::info("Attempting to create/open shared gpioregs mapping...");
    let mut gpioregs = match SharedMem::create_linked(
        sh_path.clone(), LockType::Mutex, std::mem::size_of::<RegMemory>()
    ) {
        // We created and own this mapping
        Ok(v) => v,
        // Link file already exists
        Err(SharedMemError::LinkExists) => {
            match SharedMem::open_linked(sh_path.clone()) {
                Ok(v) => v,
                Err(SharedMemError::MapOpenFailed(err)) => {
                    if n == 0 {
                        std::fs::remove_file(sh_path.clone()).unwrap_or(());
                        return _init_shared_memory(n + 1);
                    } else {
                        return Err(SharedMemError::MapOpenFailed(err));
                    }
                },
                Err(err) => return Err(err),
            }
        },
        Err(err) => return Err(err),
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
    Ok(ShMem::new(gpioregs))
}
