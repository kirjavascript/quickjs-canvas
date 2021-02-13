# sdl2-unifont [![Latest Version](https://img.shields.io/crates/v/sdl2-unifont.svg)](https://crates.io/crates/sdl2-unifont) [![Documentation](https://docs.rs/sdl2-unifont/badge.svg)](https://docs.rs/sdl2-unifont)

Sometimes, you just need to draw some text in your SDL application, and don't
need any kind of advanced rendering features. You don't want the hassle of
distributing a separate font file, relying on system fonts which often vary in
location, or having to use SDL_ttf, just to display a few lines of text.

This library embeds the GNU Unifont, xzipped, inside the application binary, and
decompresses it automatically before use. The library tries somewhat to minimise
both it's on-disk and in-memory size.

# Getting Started
Check [here](https://crates.io/crates/sdl2-unifont) for the latest version of
`sdl2-unifont`.

Or add the following to your `Cargo.toml`:
```toml
[dependencies]
sdl2-unifont = "1.0.1"
```

# Demo

<p align=center>
  <img src="https://raw.githubusercontent.com/invlpg/sdl2-unifont/master/demo.gif">
  <br><i>The example program's output</i>
</p>

Run the included example, with `cargo run --example demo --features plane-1` to
produce the above screen.

# Example
```rust
extern crate sdl2_unifont;

use sdl2_unifont::renderer::SurfaceRenderer;

use sdl2::pixels::Color;

fn main() {
    // Red text with transparent background
    let mut renderer =
        SurfaceRenderer::new(Color::RGB(255, 0, 0), Color::RGBA(0, 0, 0, 0));
        
    // Draw text to a surface, which can be used like any other. .draw() returns
    // an error result if the string contains a character which is not in the
    // Unifont.
    let surface = renderer.draw("Sample Text").unwrap();
    
    // Renderer simply holds state for producing new text surfaces
    renderer.bg_color = Color::RGB(255, 255, 0);
    renderer.bold = true;
    renderer.scale = 2;
    let example2 = renderer.draw("Big and bold").unwrap();
}
```

Consult `examples/demo.rs` for additional example code.
