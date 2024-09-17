# inspired by https://nixos.wiki/wiki/Rust
let
	pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/nixos-24.05.tar.gz")) {};
in pkgs.mkShell { 
	name = "rust-shell";
	packages = with pkgs; [ rustup openssl ];
	shellHook = ''
		rustup toolchain install stable
		rustup component add rust-analyzer
		OPENSSL_DIR=$(dirname $(which openssl))
	'';
}
