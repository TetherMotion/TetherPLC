//! Tool resolution — locate the sibling binaries `tplc` orchestrates.
//!
//! Search order for `<name>`:
//!   1. `--bindir DIR` / `$TPLC_BINDIR` (explicit override)
//!   2. directory of the running `tplc` executable (installed layouts put
//!      every tool in one bin dir)
//!   3. `PATH`
//!   4. repo-layout fallbacks: walking ancestors of the cwd and of the
//!      exe for `build/<name>` (C++ tools) and
//!      `compiler/target/{release,debug}/<name>` (Rust tools) — makes
//!      `cargo run -p tplc` and `target/release/tplc` work in-tree.

use std::path::{Path, PathBuf};

/// tool name → repo-relative candidate locations (relative to a dir that
/// looks like a repo root, i.e. contains `build/` or `compiler/`).
fn repo_candidates(root: &Path, name: &str) -> Vec<PathBuf> {
    let mut v = Vec::new();
    v.push(root.join("build").join(name));
    if name == "tplcc" || name == "tplc-pkg" || name == "tplc" {
        v.push(root.join("compiler/target/release").join(name));
        v.push(root.join("compiler/target/debug").join(name));
    }
    v
}

fn is_exe(p: &Path) -> bool {
    p.is_file()
}

fn which_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let c = dir.join(name);
        if is_exe(&c) {
            return Some(c);
        }
    }
    None
}

/// Ancestor dirs that could be a repo root: walk up from `start` while
/// the dir looks like a checkout (has `build/` or `compiler/`).
fn repo_roots(start: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut d = Some(start);
    for _ in 0..8 {
        let Some(dir) = d else { break };
        if dir.join("build").is_dir() || dir.join("compiler").is_dir() {
            out.push(dir.to_path_buf());
        }
        d = dir.parent();
    }
    out
}

#[derive(Clone, Default)]
pub struct Tools {
    pub bindir: Option<PathBuf>,
}

impl Tools {
    pub fn resolve(name: &str) -> Option<PathBuf> {
        Self::default().find(name)
    }

    pub fn find(&self, name: &str) -> Option<PathBuf> {
        // 1. explicit bindir
        let dirs: Vec<PathBuf> = {
            let mut v = Vec::new();
            if let Some(b) = &self.bindir {
                v.push(b.clone());
            }
            if let Ok(e) = std::env::var("TPLC_BINDIR") {
                if !e.is_empty() {
                    v.push(PathBuf::from(e));
                }
            }
            v
        };
        for d in &dirs {
            let c = d.join(name);
            if is_exe(&c) {
                return Some(c);
            }
        }
        // 2. exe-adjacent
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let c = dir.join(name);
                if is_exe(&c) {
                    return Some(c);
                }
            }
            // 3. repo roots relative to the exe (cargo target/release/tplc
            //    → ../../.. is the repo root)
            if let Some(dir) = exe.parent() {
                for root in repo_roots(dir) {
                    for c in repo_candidates(&root, name) {
                        if is_exe(&c) {
                            return Some(c);
                        }
                    }
                }
            }
        }
        // 4. repo roots relative to cwd
        if let Ok(cwd) = std::env::current_dir() {
            for root in repo_roots(&cwd) {
                for c in repo_candidates(&root, name) {
                    if is_exe(&c) {
                        return Some(c);
                    }
                }
            }
        }
        // 5. PATH
        which_on_path(name)
    }

    pub fn require(&self, name: &str) -> Result<PathBuf, String> {
        self.find(name).ok_or_else(|| {
            format!(
                "tplc: cannot find `{name}` — build it or pass --bindir / set TPLC_BINDIR"
            )
        })
    }

    /// Best-effort repo root (for default --libdir lists): first ancestor
    /// of cwd/exe containing `libraries/`.
    pub fn repo_root(&self) -> Option<PathBuf> {
        let mut starts = Vec::new();
        if let Ok(cwd) = std::env::current_dir() {
            starts.push(cwd);
        }
        if let Ok(exe) = std::env::current_dir().and_then(|_| std::env::current_exe()) {
            if let Some(d) = exe.parent() {
                starts.push(d.to_path_buf());
            }
        }
        for mut s in starts {
            for _ in 0..8 {
                if s.join("libraries").is_dir() {
                    return Some(s.clone());
                }
                match s.parent() {
                    Some(p) => s = p.to_path_buf(),
                    None => break,
                }
            }
        }
        None
    }
}

/// Run a tool binary with inherited stdio; returns its exit status.
pub fn run_tool(tools: &Tools, name: &str, args: &[String]) -> i32 {
    match tools.find(name) {
        Some(p) => match std::process::Command::new(&p).args(args).status() {
            Ok(st) => st.code().unwrap_or(1),
            Err(e) => {
                eprintln!("tplc: spawn {}: {e}", p.display());
                1
            }
        },
        None => {
            eprintln!("tplc: tool `{name}` not found");
            127
        }
    }
}
