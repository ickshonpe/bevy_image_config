# bevy_image_config

[![crates.io](https://img.shields.io/crates/v/bevy_image_config)](https://crates.io/crates/bevy_image_config)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_image_config)
[![crates.io](https://img.shields.io/crates/d/bevy_image_config)](https://crates.io/crates/bevy_image_config)

Bevy plugin that automatically loads and applies image sampler settings for image assets from an accompanying configuration file.

## Usage

Add the dependency to `Cargo.toml`:

```toml
bevy_image_config = "0.2"
```

Add the plugin to your app:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_image_config::ImageConfigPlugin)
        // ..rest of app
        .run()
}
```
If you have an image file "image_file_name.img_ext" and a configuration file "image_file_name.img_cfg" in the same directory, the plugin will automatically load and apply the settings from "image_file_name.img_cfg.ron" when you load "image_file_name.img_ext".

The available settings are:

* address_mode_u, address_mode_v, address_mode_w:
    ```
    ClampToEdge | Repeat | MirrorRepeat | ClampToBorder
    ```
* min_filter, mag_filter, mipmap_filter: 
    ``` 
    Nearest | Linear 
    ```
* lod_min_clamp: `f32`,
* lod_max_clamp: `f32`,
* compare:
    ```
    Option of Never | Less | Equal | LessEqual | Greater | NotEqual | GreaterEqual | Always
    ```
* anisotropy_clamp: `Option<NonZeroU8>`,
* border_color: 
    ```
    Option of TransparentBlack | OpaqueBlack | OpaqueWhite | Zero
    ```

## Examples

```
cargo run --example example
```