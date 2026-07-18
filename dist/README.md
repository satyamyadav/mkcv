# Prebuilt binaries

This directory holds prebuilt `mkcv` binaries, one per platform, committed
to the repo so the [skill](../skills/mkcv/) can download and run one
without a Rust toolchain.

```
dist/<rust-target-triple>/mkcv
```

## Adding / refreshing a binary

Build on the target platform and commit the result:

```bash
make dist          # builds --release and copies into dist/<host-target>/
```

`make dist` uses the host's Rust target triple (e.g.
`x86_64-unknown-linux-gnu`). To cover another platform, run `make dist` on that
OS/arch and commit its `dist/<target>/mkcv`.

## Notes

- The binary is large (~37 MB) — almost entirely the embedded Typst engine. A
  Git-LFS setup is a reasonable future improvement; not required for v1.
- Multi-platform release automation (GitHub Actions) and an `npx` wrapper are
  planned for **v2**; v1 ships whatever targets are built and committed here.
- The Linux binary is currently a glibc (`gnu`) build; a fully static `musl`
  build (portable across distros) needs the musl target + linker installed.
