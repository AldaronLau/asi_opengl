# [Aldaron's System Interface / OpenGL](https://crates.io/crates/asi_opengl)

This project has been discontinued, I suggest you use
[wgpu](https://crates.io/crates/wgpu) instead.

Contact me at <aldaronlau@gmail.com> if you wish to reclaim the crate name.

-----

Rust safe bindings for OpenGL / OpenGLES.

This project is part of [ADI](https://crates.io/crates/adi).

## Features
* Safe OpenGL/ES bindings (Works on both Linux (through XCB) and Windows).

## Roadmap to 1.0 (Future Features)
* Make part of `awi`.
* Support Android.
* Support Wayland.

## Change Log
### 0.6
* Pixels in textures are now represented as 4 u8s instead of 1 u32.

### 0.5
* `set_mat4` no longer takes a reference to the matrix.

### 0.4
* Safe API
* Use LINEAR instead of NEAREST for texturing.
* Use mipmapping to speed up drawing big textures in small areas.

### 0.3
* Uses sliced triangle fans now.

### 0.2
* Support for OpenGL on Linux actually works now.

### 0.1
* Initial release
