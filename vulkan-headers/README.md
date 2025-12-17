# `vulkan-headers` for Rust

This library contains minimalist Rust FFI bindings for Vulkan in a way that's
roughly equivalent to the Khronos Group's official
[Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) for C/C++. It
makes no attempt at providing safe or idiomatic Rust wrappers and doesn't rename
any C identifiers to match Rust's style guidelines.

For example, the following Rust code:

```rust
use vulkan_headers::vulkan::vulkan::*;
```

is roughly equivalent to the following C code:

```c
#include <vulkan/vulkan.h>
```

## `NonNull` Types

Unlike the C/C++ headers, this library provides `NonNull` type aliases for
function pointer types and handle types.

For example, `PFN_vkCreateInstance` is defined as:

```rust
pub type NonNullPFN_vkCreateInstance = unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
// ...
pub type PFN_vkCreateInstance = Option<NonNullPFN_vkCreateInstance>;
```

and `VkInstance` is defined as:

```rust
pub type NonNullVkInstance = NonNull<VkInstance_T>;
// ...
pub type VkInstance = *mut VkInstance_T;
```

## Optional Features

External function declarations aren't provided by default, only their
corresponding function pointer type aliases (e.g. `vkCreateInstance` isn't
provided but `PFN_vkCreateInstance` is). This is similar to defining
`VK_NO_PROTOTYPES` before including the C/C++ headers. Use the `prototypes`
feature to enable these function declarations.

The `exported_prototypes` feature can be used instead to limit these
declarations to exported functions only. This is similar to defining
`VK_ONLY_EXPORTED_PROTOTYPES` before including the C/C++ headers.

Beta (a.k.a. "provisional") extensions and/or platform-specific extensions can
be enabled using the `beta_extensions` and/or `<platform>_extensions` (e.g.
`win32_extensions`) features. This is similar to defining
`VK_ENABLE_BETA_EXTENSIONS` and/or `VK_USE_PLATFORM_<PLATFORM>` (e.g.
`VK_USE_PLATFORM_WIN32_KHR`) before including the C/C++ headers.

## Requirements

This library only supports targets with 64-bit pointers. This allows pointer and
`NonNull` types to be used portably for all Vulkan handle types.
