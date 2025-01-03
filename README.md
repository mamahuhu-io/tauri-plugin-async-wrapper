![async-wrapper](banner.svg)

Configurable async wrapper for your Tauri app.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Install

_This plugin requires a Rust version of at least **1.77.2**_

There are two general methods of installation that we can recommend.

1.Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2.Pull sources directly from Github using git tags / revision hashes (most secure)
3.Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-async-wrapper = "0.1.2"
# alternatively with Git:
tauri-plugin-log = { git = "https://github.com/mamahuhu-io/tauri-plugin-async-wrapper.git", branch = "main" }
```

## Usage

Import the Macro:
- In your Rust file, import the procedural macro:
```rust
use tauri_plugin_async_wrapper::async_wrapper;
```

Annotate Blocking Functions
- Apply the `#[async_wrapper]` macro to any blocking function you want to offload to a background thread. The function will automatically be wrapped as an asynchronous Tauri command.
```rust
#[async_blocking]
fn heavy_task(input: String) -> Result<String, String> {
    // Simulate a heavy computation or I/O operation
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(format!("Processed: {}", input))
}

```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License

Code: (c) 2015 - Present - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.