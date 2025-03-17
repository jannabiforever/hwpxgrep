use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum TextKind {
    T,
    Script,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub kind: TextKind,
    pub inner: String,
}

impl Text {
    pub fn new(kind: TextKind, inner: String) -> Self {
        Self { kind, inner }
    }

    pub fn new_t(inner: String) -> Self {
        Self {
            kind: TextKind::T,
            inner,
        }
    }

    pub fn new_script(inner: String) -> Self {
        Self {
            kind: TextKind::Script,
            inner,
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
