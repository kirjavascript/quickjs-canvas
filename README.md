try the examples;

```bash
cargo run --release -- --file examples/hello.js
```

### development dependencies

`libsdl2-dev libsdl2-gfx-dev liblzma-dev`

### TODO

* support useful canvas API subset
    * improve text implementation (try orbfont)
    * arc
    * curves
* events
* fix window closing
* examples: zdog, d3-force, window.moveTo breakout
* document features
* object-inspect for debugging
* fetch
* clipboard
* IO
* localStorage
* btoa atob
* setTimeout / setInterval
* support full canvas API
* file watching (hot reload)
* Path2d
* cross compile for windows
* --png
* --embed script.js --out file.exe
* make "bundled", "static-link" work with sdl2_gfx
* prompt() from window, with blocked bool
* nanovg

#### libs to consider

https://crates.io/crates/enum-as-inner
http://www.zebkit.org/dark/about.html
