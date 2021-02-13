// For unifont.rs
extern crate lzma;

// For renderer.rs
extern crate bit_field;
extern crate sdl2;

/// Manages Unifont initialisation and rendering; the main API interface
pub mod renderer;

/// Low-level global Unifont storage. You shouldn't need to interface with this
pub mod unifont;
