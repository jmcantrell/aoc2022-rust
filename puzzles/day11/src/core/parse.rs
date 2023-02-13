use anyhow::Context;

pub fn binary<'a>(s: &'a str, sep: &'a str) -> anyhow::Result<(&'a str, &'a str)> {
    let mut split = s.trim().splitn(2, sep);

    let left = split.next().context("missing left hand side")?.trim();
    let right = split.next().context("missing right hand side")?.trim();

    Ok((left, right))
}

pub fn prefix<'a>(s: &'a str, prefix: &'a str) -> anyhow::Result<&'a str> {
    Ok(s.trim()
        .strip_prefix(prefix)
        .with_context(|| format!("expected string to start with {prefix:?}"))?
        .trim())
}
