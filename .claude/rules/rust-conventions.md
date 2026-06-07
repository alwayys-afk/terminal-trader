There is a pre-commit hook that runs `cargo fmt --all`. If a commit fails due to formatting, run `cargo fmt --all`, re-stage, and commit again.

Avoid cloning when it's not necessary. Prefer borrowing (`&T`, `&str`, `&[T]`) over `.clone()` unless ownership transfer is actually needed.
