use anyhow::Context;

pub(crate) fn ensure_prefix<'a>(s: &'a str, prefix: &'a str) -> anyhow::Result<&'a str> {
    Ok(s.trim()
        .strip_prefix(prefix)
        .with_context(|| format!("expected string to start with {prefix:?}"))?
        .trim())
}
