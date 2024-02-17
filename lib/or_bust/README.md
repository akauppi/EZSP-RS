# `or_bust`

Provides a method for booleans in Rust to be asserted, and errors generated, with the boolean leading the way.

**Instead of:**

```
if !(len < 4) return Err([...snip...] format!("too short: {len} < 4"));
```

```
assert!(len < 4, "too short: {} < 4", len);
```

**..this:**

```
(len < 4).or_bust(|| format!("too short: {len} < 4"))?;
```

## Why this matters?

It's nice to get the positive assumption up front (leftmost on the line). It eases readability.

The author hasn't found a Rust built-in way to achieve this.

## Using

You can use this package by either:

- making a git submodule in your repo, and using as a local dependency
- adding the path to git repo to your `Cargo.toml`

The author isn't intending to keep maintaining this - it's just a snippet.
