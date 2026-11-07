//! Shared frontend diagnostics — product-layer shape (doc 05 §12):
//! code, severity, primary span, message, optional help. Both
//! frontends (ST, FBD) emit this type so downstream tooling sees one
//! diagnostic surface.

use crate::ir::Prov;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Diag {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub help: Option<String>,
    /// secondary messages attached under the primary span
    pub notes: Vec<String>,
    pub span: Prov,
}

impl Diag {
    pub fn error(code: &'static str, message: impl Into<String>, span: Prov) -> Self {
        Diag {
            code,
            severity: Severity::Error,
            message: message.into(),
            help: None,
            span,
            notes: Vec::new(),
        }
    }
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// rustc-style rendering: source line + caret span + help/notes.
    /// `src` = the text of the file the `Prov::St{file}` index refers to.
    pub fn render(&self, file_name: &str, src: &str) -> String {
        let mut out = format!("{:?}[{}] {}", self.severity, self.code, self.message);
        if let Prov::Fbd { sheet, node, pin } = self.span {
            out += &format!("\n  --> fbd sheet {sheet} node {node} pin {pin}");
            if let Some(h) = &self.help {
                out += &format!("\n  = help: {h}");
            }
            for n in &self.notes {
                out += &format!("\n  = note: {n}");
            }
            return out;
        }
        let Prov::St { offset, len, .. } = self.span else {
            if let Some(h) = &self.help {
                out += &format!("\n  = help: {h}");
            }
            for n in &self.notes {
                out += &format!("\n  = note: {n}");
            }
            return out;
        };
        let (off, ln) = (offset as usize, len.max(1) as usize);
        let mut line = 1usize;
        let mut col = 1usize;
        let mut line_start = 0usize;
        for (i, c) in src.char_indices() {
            if i >= off {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
                line_start = i + 1;
            } else {
                col += 1;
            }
        }
        let line_end = src[line_start..]
            .find('\n')
            .map(|e| line_start + e)
            .unwrap_or(src.len());
        let text = &src[line_start..line_end];
        let lnw = line.to_string().len().max(2);
        out += &format!("\n{:>w$}--> {}:{line}:{col}", "", file_name, w = lnw + 1);
        out += &format!("\n{:>w$} |", "", w = lnw);
        out += &format!("\n{line:>w$} | {text}", w = lnw);
        let caret = "^".repeat(ln.min(text.len().saturating_sub(col - 1)).max(1));
        out += &format!(
            "\n{:>w$} | {:>pad$}{}",
            "",
            "",
            caret,
            w = lnw,
            pad = col - 1
        );
        if let Some(h) = &self.help {
            out += &format!("\n{:>w$} = help: {h}", "", w = lnw);
        }
        for n in &self.notes {
            out += &format!("\n{:>w$} = note: {n}", "", w = lnw);
        }
        out
    }

    pub fn warning(code: &'static str, message: impl Into<String>, span: Prov) -> Self {
        Diag {
            code,
            severity: Severity::Warning,
            message: message.into(),
            help: None,
            notes: Vec::new(),
            span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "PROGRAM P\nVAR x : INT; END_VAR\ny := x;\nEND_PROGRAM\n";

    #[test]
    fn render_shows_line_col_caret_and_notes() {
        let d = Diag::error(
            "TPLC_E_ST_UNKNOWN_NAME",
            "undeclared name y",
            Prov::St {
                file: 0,
                offset: 31,
                len: 1,
            },
        )
        .with_help("did you mean `x`?")
        .with_note("declared names: x");
        let r = d.render("p.st", SRC);
        assert!(r.contains("--> p.st:3:1"));
        assert!(r.contains("3 | y := x;"));
        assert!(r.contains("^"));
        assert!(r.contains("= help: did you mean `x`?"));
        assert!(r.contains("= note: declared names: x"));
    }

    #[test]
    fn render_fbd_provenance() {
        let d = Diag::error(
            "TPLC_E_ST_TYPE",
            "type mismatch",
            Prov::Fbd {
                sheet: 0,
                node: 7,
                pin: 2,
            },
        );
        let r = d.render("<fbd>", "");
        assert!(r.contains("fbd sheet 0 node 7 pin 2"));
    }
}
