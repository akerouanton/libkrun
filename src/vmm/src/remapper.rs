use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
pub fn start_remapper_thread(vmm: Arc<Mutex<super::Vmm>>) -> io::Result<()> {
    thread::Builder::new()
        .name("mem remapper".into())
        .spawn(move || loop {
            vmm.lock().unwrap().remap_guest_mem();
            thread::sleep(Duration::from_secs(5));
        })?;
    Ok(())
}
