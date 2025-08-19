{
  mkShell,
  rust-analyzer,
  rustfmt,
  rustc,
  clippy,
  cargo,
  rustPlatform,
}:
mkShell {
  name = "rust";
  packages = [
    rust-analyzer
    rustfmt
    clippy
    cargo
    rustc
  ];

  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
