//! Project model — per-directory persistence for tplc.
//!
//! A *project* is a directory containing `.tplc/project.json` (the
//! manifest).  Everything lives in that directory — sources, local
//! libraries, build artifacts (`build/`), the manifest, and an
//! integrated git repository (`.git/`, created by `tplc new`/`init`).
//! No central database: discovery walks up from the cwd looking for
//! `.tplc/project.json`; a small recent-projects list in the runtime
//! state dir is only a UI convenience.
//!
//! Manifest schema (`.tplc/project.json`, version 1):
//! ```json
//! {
//!   "version": 1,
//!   "name": "oven",
//!   "description": "…",
//!   "sources": ["src/main.st"],
//!   "libs": ["libs", "../TetherPLC/libraries/tempctrl"],
//!   "taskPeriodNs": 10000000,
//!   "imageIn": 64, "imageOut": 64,
//!   "deployMap": "deploy.map",
//!   "ui": { …opaque WebUI document… }
//! }
//! ```
//! Unknown keys are preserved on load/save (forward compatibility).

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Component, Path, PathBuf};

pub const TPL_DIR: &str = ".tplc";
pub const MANIFEST_NAME: &str = "project.json";
pub const MANIFEST_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Default sources for `tplc run|sim|dev` when no positional
    /// sources/packages are given (paths relative to project root).
    #[serde(default)]
    pub sources: Vec<String>,
    /// Extra `--lib` dirs for tplcc (relative to project root).
    #[serde(default)]
    pub libs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_period_ns: Option<u64>,
    #[serde(default)]
    pub image_in: u32,
    #[serde(default)]
    pub image_out: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deploy_map: Option<String>,
    /// Opaque WebUI document (the PlcProject JSON the IDE edits).
    /// The backend stores it verbatim; the UI owns the schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui: Option<Value>,
    /// Forward-compat: keys this version doesn't know are kept.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

fn default_version() -> u32 {
    MANIFEST_VERSION
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            version: MANIFEST_VERSION,
            name: String::new(),
            description: None,
            sources: vec![],
            libs: vec![],
            task_period_ns: None,
            image_in: 64,
            image_out: 64,
            deploy_map: None,
            ui: None,
            extra: Map::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Project {
    /// Absolute project root (the dir containing `.tplc/`).
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub fn tpl_dir(&self) -> PathBuf {
        self.root.join(TPL_DIR)
    }
    pub fn manifest_path(&self) -> PathBuf {
        self.tpl_dir().join(MANIFEST_NAME)
    }
    /// Build artifacts live here — gitignored by scaffold.
    pub fn build_dir(&self) -> PathBuf {
        self.root.join("build")
    }

    pub fn display_name(&self) -> &str {
        if self.manifest.name.is_empty() {
            self.root
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("project")
        } else {
            &self.manifest.name
        }
    }

    /// Resolve a manifest-relative path to an absolute one.
    pub fn resolve(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let dir = self.tpl_dir();
        std::fs::create_dir_all(&dir)?;
        let txt = serde_json::to_string_pretty(&self.manifest)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        // atomic-ish: write tmp + rename so a crash can't truncate
        let tmp = dir.join(".project.json.tmp");
        std::fs::write(&tmp, format!("{txt}\n"))?;
        std::fs::rename(&tmp, self.manifest_path())
    }
}

/// Walk up from `start` looking for `.tplc/project.json`.
/// Returns the project root on the first hit.
pub fn discover(start: &Path) -> Option<PathBuf> {
    let mut d = Some(start);
    for _ in 0..32 {
        let dir = d?;
        if dir.join(TPL_DIR).join(MANIFEST_NAME).is_file() {
            return Some(dir.to_path_buf());
        }
        d = dir.parent();
    }
    None
}

/// Load the project rooted at `root` (must contain `.tplc/project.json`).
pub fn load(root: &Path) -> Result<Project, String> {
    let mpath = root.join(TPL_DIR).join(MANIFEST_NAME);
    let txt = std::fs::read_to_string(&mpath)
        .map_err(|e| format!("{}: {e}", mpath.display()))?;
    let manifest: Manifest = serde_json::from_str(&txt)
        .map_err(|e| format!("{}: bad manifest: {e}", mpath.display()))?;
    if manifest.version > MANIFEST_VERSION {
        return Err(format!(
            "{}: manifest version {} > supported {}",
            mpath.display(),
            manifest.version,
            MANIFEST_VERSION
        ));
    }
    Ok(Project {
        root: root.to_path_buf(),
        manifest,
    })
}

/// Discover from `start` (default: cwd) and load it.
pub fn open(start: Option<&Path>) -> Result<Project, String> {
    let cwd = match start {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().map_err(|e| format!("cwd: {e}"))?,
    };
    let root = discover(&cwd).ok_or_else(|| {
        format!(
            "no tplc project found above {} — create one with `tplc new <dir>` or `tplc init`",
            cwd.display()
        )
    })?;
    load(&root)
}

/// Default manifest name for a fresh project: dir basename, sanitized.
pub fn default_name(dir: &Path) -> String {
    dir.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project")
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

const SCAFFOLD_ST: &str = "\
// main.st — scaffolded by `tplc new`; blink a %Q output at ~1 Hz.
PROGRAM Main
VAR
    tick AT %QX0.0 : BOOL;
    t    : TIME;
END_VAR
t := t + T#50ms;
IF t >= T#1s THEN
    t := T#0ms;
    tick := NOT tick;
END_IF;
END_PROGRAM
";

const GITIGNORE: &str = "\
# tplc project — build artifacts are not source
build/
.tplc/cache/
*.o
*.so
";

/// Create (or adopt) a project in `dir`: `.tplc/project.json`,
/// `.gitignore`, a scaffold `src/main.st` when no sources exist,
/// and a git repo (via crate::git::init).  Records the project in the
/// recent list.  Returns the loaded project.
pub fn init(dir: &Path, name: Option<&str>) -> Result<Project, String> {
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?;
    let root = dir
        .canonicalize()
        .unwrap_or_else(|_| dir.to_path_buf());

    // load existing manifest if present, else default
    let mpath = root.join(TPL_DIR).join(MANIFEST_NAME);
    let mut manifest = if mpath.is_file() {
        load(&root)?.manifest
    } else {
        let mut m = Manifest::default();
        m.name = name
            .map(|s| s.to_string())
            .unwrap_or_else(|| default_name(&root));
        m.sources = vec!["src/main.st".into()];
        m
    };
    if let Some(n) = name {
        manifest.name = n.to_string();
    }
    if manifest.name.is_empty() {
        manifest.name = default_name(&root);
    }

    // scaffold source if the project has none yet
    let src_dir = root.join("src");
    let main_st = src_dir.join("main.st");
    if manifest.sources.is_empty() || !manifest.sources.iter().any(|s| {
        let p = root.join(s);
        p.is_file()
    }) {
        std::fs::create_dir_all(&src_dir)
            .map_err(|e| format!("{}: {e}", src_dir.display()))?;
        if !main_st.exists() {
            std::fs::write(&main_st, SCAFFOLD_ST)
                .map_err(|e| format!("{}: {e}", main_st.display()))?;
        }
        if manifest.sources.is_empty() {
            manifest.sources = vec!["src/main.st".into()];
        }
    }

    // gitignore (don't clobber a user's)
    let gi = root.join(".gitignore");
    if !gi.exists() {
        std::fs::write(&gi, GITIGNORE).ok();
    }

    let proj = Project {
        root: root.clone(),
        manifest,
    };
    proj.save().map_err(|e| e.to_string())?;

    // integrated git — init if needed, then an initial commit when the
    // repo has no commits yet (manual-commit policy: we only make this
    // one so `tplc git log` isn't empty).
    match crate::git::ensure_repo(&root) {
        Ok(true) => {
            let _ = crate::git::add_all(&root);
            let _ = crate::git::commit(&root, "tplc: initial project");
        }
        Ok(false) => {}
        Err(e) => eprintln!("tplc: git init: {e}"),
    }

    record_recent(&root);
    Ok(proj)
}

/* ---------------- recent projects (UI convenience only) ----------- */

fn recent_path() -> PathBuf {
    crate::session::state_dir().join("projects.json")
}

/// Record `root` as the most-recently-used project (dedup, cap 20).
pub fn record_recent(root: &Path) {
    let p = recent_path();
    let mut list: Vec<String> = std::fs::read_to_string(&p)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default();
    let s = root.display().to_string();
    list.retain(|x| x != &s);
    list.insert(0, s);
    list.truncate(20);
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let _ = std::fs::write(&p, serde_json::to_string_pretty(&list).unwrap_or_default());
}

/// Recently opened project roots, most-recent first.
pub fn recent() -> Vec<PathBuf> {
    std::fs::read_to_string(recent_path())
        .ok()
        .and_then(|t| serde_json::from_str::<Vec<String>>(&t).ok())
        .unwrap_or_default()
        .into_iter()
        .map(PathBuf::from)
        .filter(|p| p.join(TPL_DIR).join(MANIFEST_NAME).is_file())
        .collect()
}

/* ---------------- path safety (serve file endpoints) -------------- */

/// Join `rel` onto `root`, rejecting absolute paths, `..` escapes and
/// NUL bytes.  Returns None for anything that would land outside root.
pub fn safe_join(root: &Path, rel: &str) -> Option<PathBuf> {
    if rel.is_empty() || rel.contains('\0') {
        return None;
    }
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() {
        return None;
    }
    let mut out = root.to_path_buf();
    for c in rel_path.components() {
        match c {
            Component::Normal(s) => out.push(s),
            Component::CurDir => {}
            _ => return None, // ParentDir or RootDir/Prefix
        }
    }
    // final canonical check when the file exists (symlink escape guard)
    if let Ok(canon) = out.canonicalize() {
        let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        if !canon.starts_with(&root_canon) {
            return None;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "tplc-proj-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn manifest_roundtrip_preserves_unknown_keys() {
        let d = tmpdir("rt");
        let mut p = init(&d, Some("rt")).unwrap();
        // inject an unknown key + ui doc, save, reload
        p.manifest
            .extra
            .insert("futureField".into(), Value::from(42));
        let mut m2 = p.manifest.clone();
        m2.ui = Some(serde_json::json!({"fbd": {"pous": []}}));
        let proj2 = Project {
            root: p.root.clone(),
            manifest: m2,
        };
        proj2.save().unwrap();
        let back = load(&p.root).unwrap();
        assert_eq!(back.manifest.name, "rt");
        assert_eq!(back.manifest.extra["futureField"], 42);
        assert!(back.manifest.ui.is_some());
        assert_eq!(back.manifest.sources, vec!["src/main.st".to_string()]);
        assert!(p.root.join("src/main.st").is_file());
        assert!(p.root.join(".git").is_dir() || !crate::git::available());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn discover_walks_up() {
        let d = tmpdir("disc");
        init(&d, None).unwrap();
        let deep = d.join("a/b/c");
        std::fs::create_dir_all(&deep).unwrap();
        assert_eq!(discover(&deep), Some(d.canonicalize().unwrap()));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn safe_join_rejects_escape() {
        let d = tmpdir("safe");
        init(&d, None).unwrap();
        let root = d.canonicalize().unwrap();
        assert!(safe_join(&root, "src/main.st").is_some());
        assert!(safe_join(&root, "../outside").is_none());
        assert!(safe_join(&root, "/etc/passwd").is_none());
        assert!(safe_join(&root, "a/../../b").is_none());
        assert!(safe_join(&root, "").is_none());
        let _ = std::fs::remove_dir_all(&d);
    }
}
