try the examples;

```bash
cargo run --release -- --file examples/hello.js
```

### development dependencies

`libsdl2-dev liblzma-dev`

### TODO

* performance.now() / first param to rAF
* support useful canvas API subset
    * paths
    * text
* fix window closing
* alert, prompt, confirm (use last focused window)
* examples: zdog, d3-force
* document features
* events
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
* --embed script.js --out file.exe
* make "bundled", "static-link" work with sdl2_gfx
* nanovg
* --png
* http://www.zebkit.org/dark/about.html
