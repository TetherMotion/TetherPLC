//! Integrated git for tplc projects — thin git2 wrapper.
//!
//! Every project dir gets a `.git/` repo on `tplc new`/`init`
//! (manual-commit policy: the only automatic commit is the initial
//! scaffold commit).  All functions degrade gracefully: when `dir`
//! isn't a repo or git2 fails, callers get `Ok(false)`/`Err` with a
//! readable message — never a panic.

use std::path::Path;

/// True when a git repository exists at or above `dir`.
pub fn is_repo(dir: &Path) -> bool {
    git2::Repository::discover(dir).is_ok()
}

/// Whether git2/libgit2 is usable at all in this build.
#[allow(dead_code)] // probe for tests/diagnostics
pub fn available() -> bool {
    // libgit2 is always linked; this exists so tests/UI can probe.
    true
}

/// Ensure `dir` is inside a git work tree; init one at `dir` when not.
/// Returns Ok(true) when a NEW repo was created.
pub fn ensure_repo(dir: &Path) -> Result<bool, String> {
    if is_repo(dir) {
        return Ok(false);
    }
    git2::Repository::init(dir).map_err(|e| format!("git init: {e}"))?;
    // sane defaults for a PLC project: no signing, utf8
    if let Ok(mut cfg) = git2::Repository::open(dir).and_then(|r| r.config()) {
        let _ = cfg.set_str("core.autocrlf", "input");
    }
    Ok(true)
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct StatusEntry {
    pub path: String,
    /// 'M' modified, 'A' added/staged-new, 'D' deleted, 'R' renamed,
    /// '?' untracked, ' ' unchanged-context (not emitted).
    pub status: String,
    pub staged: bool,
    pub index_status: String,
    pub workdir_status: String,
}

/// Porcelain status of the repo containing `dir`.
pub fn status(dir: &Path) -> Result<Vec<StatusEntry>, String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    let mut so = git2::StatusOptions::new();
    so.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        .rename_threshold(50);
    let statuses = repo
        .statuses(Some(&mut so))
        .map_err(|e| format!("git status: {e}"))?;
    let mut out = Vec::new();
    for e in statuses.iter() {
        let s = e.status();
        let mut tags = String::new();
        let mut code = |f: git2::Status, c: char| {
            if s.contains(f) {
                tags.push(c);
            }
        };
        code(git2::Status::INDEX_NEW, 'A');
        code(git2::Status::INDEX_MODIFIED, 'M');
        code(git2::Status::INDEX_DELETED, 'D');
        code(git2::Status::INDEX_RENAMED, 'R');
        code(git2::Status::WT_NEW, '?');
        code(git2::Status::WT_MODIFIED, 'M');
        code(git2::Status::WT_DELETED, 'D');
        code(git2::Status::WT_RENAMED, 'R');
        if tags.is_empty() {
            tags.push(' ');
        }
        out.push(StatusEntry {
            path: e.path().unwrap_or("").to_string(),
            status: tags,
            staged: s.intersects(
                git2::Status::INDEX_NEW
                    | git2::Status::INDEX_MODIFIED
                    | git2::Status::INDEX_DELETED
                    | git2::Status::INDEX_RENAMED
                    | git2::Status::INDEX_TYPECHANGE,
            ),
            index_status: idx_code(s),
            workdir_status: wt_code(s),
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

fn idx_code(s: git2::Status) -> String {
    if s.contains(git2::Status::INDEX_NEW) {
        "A".into()
    } else if s.contains(git2::Status::INDEX_MODIFIED) {
        "M".into()
    } else if s.contains(git2::Status::INDEX_DELETED) {
        "D".into()
    } else if s.contains(git2::Status::INDEX_RENAMED) {
        "R".into()
    } else {
        " ".into()
    }
}

fn wt_code(s: git2::Status) -> String {
    if s.contains(git2::Status::WT_NEW) {
        "?".into()
    } else if s.contains(git2::Status::WT_MODIFIED) {
        "M".into()
    } else if s.contains(git2::Status::WT_DELETED) {
        "D".into()
    } else if s.contains(git2::Status::WT_RENAMED) {
        "R".into()
    } else {
        " ".into()
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CommitInfo {
    pub oid: String,
    pub short: String,
    pub author: String,
    pub email: String,
    pub time: i64, // unix seconds
    pub message: String,
}

/// Recent commits, newest first.
pub fn log(dir: &Path, max: usize) -> Result<Vec<CommitInfo>, String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return Ok(vec![]), // unborn branch — no commits yet
    };
    let mut rev = head.peel_to_commit().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    loop {
        let (oid, short, author, email, time, message) = {
            let sig = rev.author();
            (
                rev.id().to_string(),
                rev.id().to_string()[..7].to_string(),
                sig.name().unwrap_or("").to_string(),
                sig.email().unwrap_or("").to_string(),
                sig.when().seconds(),
                rev.message()
                    .unwrap_or("")
                    .lines()
                    .next()
                    .unwrap_or("")
                    .to_string(),
            )
        };
        out.push(CommitInfo {
            oid,
            short,
            author,
            email,
            time,
            message,
        });
        if out.len() >= max {
            break;
        }
        match rev.parent(0) {
            Ok(p) => rev = p,
            Err(_) => break,
        }
    }
    Ok(out)
}

/// Unified diff of `path` (None = whole tree) against HEAD.
/// Returns None when the repo has no commits yet (diff vs empty tree
/// would be huge; callers show status instead).
pub fn diff(dir: &Path, path: Option<&str>) -> Result<Option<String>, String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    let head = match repo.head().and_then(|h| h.peel_to_commit()) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };
    let tree = head.tree().map_err(|e| e.to_string())?;
    let mut opts = git2::DiffOptions::new();
    if let Some(p) = path {
        opts.pathspec(p);
    }
    // include untracked content as new-file diffs
    let mut diff = repo
        .diff_tree_to_workdir_with_index(Some(&tree), Some(&mut opts))
        .map_err(|e| format!("git diff: {e}"))?;
    diff.find_similar(None).ok();
    let mut out = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_d, _h, line| {
        out.extend_from_slice(line.content());
        true
    })
    .map_err(|e| format!("git diff print: {e}"))?;
    Ok(Some(String::from_utf8_lossy(&out).into_owned()))
}

/// Stage everything under `dir` (respecting .gitignore).
pub fn add_all(dir: &Path) -> Result<(), String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    let mut idx = repo.index().map_err(|e| e.to_string())?;
    idx.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| format!("git add: {e}"))?;
    idx.write().map_err(|e| e.to_string())
}

/// Stage `paths` (relative to repo workdir) then commit with `msg`.
/// When `paths` is empty, stages everything first.
pub fn commit(dir: &Path, msg: &str) -> Result<String, String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    if msg.trim().is_empty() {
        return Err("empty commit message".into());
    }
    let mut idx = repo.index().map_err(|e| e.to_string())?;
    idx.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| format!("git add: {e}"))?;
    idx.write().map_err(|e| e.to_string())?;
    let tree_id = idx.write_tree().map_err(|e| e.to_string())?;
    let tree = repo.find_tree(tree_id).map_err(|e| e.to_string())?;
    let sig = git2::Signature::now(
        &std::env::var("GIT_AUTHOR_NAME").unwrap_or_else(|_| {
            whoami_fallback()
        }),
        &std::env::var("GIT_AUTHOR_EMAIL").unwrap_or_else(|_| {
            format!("{}@localhost", whoami_fallback())
        }),
    )
    .map_err(|e| format!("git signature: {e}"))?;
    let parents: Vec<git2::Commit> = match repo.head().and_then(|h| h.peel_to_commit()) {
        Ok(c) => vec![c],
        Err(_) => vec![],
    };
    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
    let oid = repo
        .commit(Some("HEAD"), &sig, &sig, msg, &tree, &parent_refs)
        .map_err(|e| format!("git commit: {e}"))?;
    Ok(oid.to_string())
}

fn whoami_fallback() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "tplc".into())
}

/// Current branch name ("HEAD" detached → short oid or "(detached)").
pub fn branch(dir: &Path) -> Result<String, String> {
    let repo = git2::Repository::discover(dir)
        .map_err(|e| format!("not a git repo: {e}"))?;
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return Ok("(unborn)".into()),
    };
    Ok(head.shorthand().unwrap_or("HEAD").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_status_commit_log_roundtrip() {
        let d = std::env::temp_dir().join(format!(
            "tplc-git-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();

        assert!(!is_repo(&d));
        assert!(ensure_repo(&d).unwrap());
        assert!(is_repo(&d));
        assert!(ensure_repo(&d).unwrap() == false);

        std::fs::write(d.join("a.st"), "PROGRAM A END_PROGRAM\n").unwrap();
        let st = status(&d).unwrap();
        assert!(st.iter().any(|e| e.path == "a.st" && e.status.contains('?')));

        commit(&d, "first").unwrap();
        let commits = log(&d, 10).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].message, "first");
        assert_eq!(branch(&d).unwrap(), "master");

        // modify → status shows M; diff non-empty
        std::fs::write(d.join("a.st"), "PROGRAM A VAR x:INT; END_VAR END_PROGRAM\n").unwrap();
        let st = status(&d).unwrap();
        assert!(st.iter().any(|e| e.path == "a.st" && e.workdir_status == "M"));
        let diff = diff(&d, Some("a.st")).unwrap();
        let patch = diff.unwrap();
        assert!(patch.contains("PROGRAM A VAR") && patch.contains("END_VAR"), "diff was: {patch}");

        commit(&d, "second").unwrap();
        assert_eq!(log(&d, 10).unwrap().len(), 2);

        let _ = std::fs::remove_dir_all(&d);
    }
}
