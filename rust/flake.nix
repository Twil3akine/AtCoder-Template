{
  description = "AtCoder Rust dev environment + local runner";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs     = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        # ── runner バイナリ本体 ──────────────────────────────────────
        runnerBin = pkgs.rustPlatform.buildRustPackage {
          pname   = "runner";
          version = "0.1.0";
          src     = ./runner;
          cargoLock.lockFile = ./runner/Cargo.lock;
        };

        # python3/pypy3/rustc を PATH に持つラッパー
        runner = pkgs.writeShellScriptBin "runner" ''
          export PATH="${pkgs.python3}/bin:${pkgs.pypy3}/bin:${rustToolchain}/bin:$PATH"
          exec ${runnerBin}/bin/runner "$@"
        '';

        runnerStop = pkgs.writeShellScriptBin "runner-stop" ''
          PID=$(lsof -i :4000 -t 2>/dev/null)
          if [ -n "$PID" ]; then
            kill "$PID"
            echo "Runner stopped (PID $PID)"
          else
            echo "Runner is not running"
          fi
        '';

      in {
        packages.runner = runner;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.python3
            pkgs.pypy3
            pkgs.cargo-watch
            runner
            runnerStop
          ];

          shellHook = ''
            if lsof -i :4000 -t >/dev/null 2>&1; then
              echo "Local Runner already running on http://127.0.0.1:4000"
            else
              runner > /tmp/atcoder-runner.log 2>&1 &
              echo "Local Runner started on http://127.0.0.1:4000"
            fi
          '';
        };
      }
    );
}
