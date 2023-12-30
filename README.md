# One Horn Mod Manager

## Development Setup Instructions (Linux)

Install the webkit2gtk library through your preferred method

Install the wasm target for rust `rustup target add wasm32-unknown-unknown`

Install the tauri prerequisites with:
`cargo install tauri-cli trunk`

If not done already add the cargo binaries to your PATH

Run `cargo tauri dev`