This repository contains three projects:

  * [vulkan-headers](vulkan-headers) contains minimalist Rust FFI bindings for
    Vulkan in a way that's roughly equivalent to the Khronos Group's official
    [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) for C/C++.
  * [vulkan-headers-generator](vulkan-headers-generator) uses `vulkan-registry`
    to generate the `vulkan-headers` Rust library.
  * [vulkan-registry](vulkan-registry) parses the Vulkan API Registry XML files
    into Rust data structures.
