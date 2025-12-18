# Vulkan API Registry for Rust

This library parses the Vulkan API Registry XML files into Rust data structures.

## Example

List all Vulkan platform names:

```rust
fn main() {
    use vulkan_registry::*;

    let registry = Registry::vk(); // vk.xml
    for content in &registry.contents {
        if let RegistryContent::Platforms(platforms) = content {
            for content in &platforms.contents {
                let PlatformsContent::Platform(platform) = content;
                println!("{:?}", platform.name);
            }
        }
    }
}
```
