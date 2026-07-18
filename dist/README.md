# Prebuilt binary

`dist/mkcv` is a prebuilt **Linux x86_64** (glibc) `mkcv` binary, committed to
the repo so the [skill](../skills/mkcv/) can download and run it without a Rust
toolchain.

## Refreshing it

```bash
make dist    # builds --release and copies target/release/mkcv -> dist/mkcv
```

## Notes

- **Linux x86_64 only.** On any other OS/arch, build from source
  (`cargo build --release`) and point `MKCV_BIN` at your binary — the skill's
  `install.sh --bin` does this automatically. Mac binaries
  (`dist/mkcv-macos-arm64`, `dist/mkcv-macos-x64`) are wired in and will work
  automatically once CI publishes them.
- The binary is ~40 MB, almost entirely the embedded Typst engine. Git-LFS is a
  reasonable future improvement; not required for v1.
- Multi-platform release binaries and an `npx` wrapper are planned for **v2**.
