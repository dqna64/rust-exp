# RustLang
- ## Tools
	- rustc
	- rustup
	- cargo
	- clippy
## Setup
	- **rustup**: toolchain installer
	- `$RUSTUP_HOME` env var
		- `$HOME/.rustup`
	- `$CARGO_HOME` env var
		- `$HOME/.cargo`
	- `$HOME/.cargo/bin`
		- contains `cargo`, `rustc`, `rustup` programs
		- added to `$PATH`
	- `rustup self uninstall` to revert these changes
- ## Modules
	- std::io::Stdin
	- std::print