// Serialize cmake invocations on a build dir — `cargo build`'s
// build.rs racing a manual `cmake --build` (or a second cargo) on the
// same dir corrupts the dep graph / produces sporadic failures.
//
// This file is *included* (`include!`) by `build.rs` — build scripts
// are separate compilation units that cannot use crate modules — and
// declared as a normal module for the serve auto-rebuild path. Keep it
// dependency-free (std only). NOTE: `//!`/`///` doc comments are
// ILLEGAL here — they become inner/mid-item docs once included —
// use plain `//` throughout.

// O_EXCL lockfile with stale-lock takeover; giving up the wait returns
// None and the caller decides whether to proceed anyway.
pub(crate) struct CmakeLock(std::path::PathBuf);

impl CmakeLock {
    // Wait up to `max_wait` for `dir/.tplc-cmake.lock`.
    pub(crate) fn acquire(
        dir: &std::path::Path,
        max_wait: std::time::Duration,
    ) -> Option<CmakeLock> {
        let path = dir.join(".tplc-cmake.lock");
        let start = std::time::Instant::now();
        loop {
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(_) => return Some(CmakeLock(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    let stale = std::fs::metadata(&path)
                        .and_then(|m| m.modified())
                        .map(|t| {
                            t.elapsed().unwrap_or_default()
                                > std::time::Duration::from_secs(30 * 60)
                        })
                        .unwrap_or(false);
                    if stale {
                        let _ = std::fs::remove_file(&path);
                        continue;
                    }
                    if start.elapsed() > max_wait {
                        return None;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                Err(_) => return None,
            }
        }
    }
}

impl Drop for CmakeLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
