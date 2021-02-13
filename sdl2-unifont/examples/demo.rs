/*
 * Library example code:
 */
#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate sdl2_unifont;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2_unifont::renderer::{FormattedRenderer, SurfaceRenderer};
use std::boxed::Box;
use std::time::Duration;

/// Rainbow text colours, from back to front
lazy_static! {
    static ref colours: [Box<Color>; 12] = [
        Box::new(Color::RGB(255, 0, 127)),
        Box::new(Color::RGB(255, 0, 255)),
        Box::new(Color::RGB(127, 0, 255)),
        Box::new(Color::RGB(0, 0, 255)),
        Box::new(Color::RGB(0, 127, 255)),
        Box::new(Color::RGB(0, 255, 255)),
        Box::new(Color::RGB(0, 255, 127)),
        Box::new(Color::RGB(0, 255, 0)),
        Box::new(Color::RGB(127, 255, 0)),
        Box::new(Color::RGB(255, 255, 0)),
        Box::new(Color::RGB(255, 127, 0)),
        Box::new(Color::RGB(255, 0, 0)),
    ];
}

/// Used for the FormattedRenderer demo to demonstrate persistence and variable
/// value updating (draws the text draw time string).
static mut G_FMT_RENDERER: Option<FormattedRenderer> = None;

/// Called from main to draw the demo text objects once at program start.
/// This function demonstrates the various functionality of the library.
fn draw_demo<'a>(
    iter_num: usize,
    last_time: Duration,
) -> sdl2::surface::Surface<'a> {
    // Where we'll blit all of our text surfaces onto
    let mut screen = sdl2::surface::Surface::new(
        800,
        600,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    ).unwrap();

    // Used to create surfaces containing rendered text
    let mut renderer =
        SurfaceRenderer::new(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255));

    /*
     * Simple demos
     */
    // The renderer draw method returns a surface, which we can use like any
    // other SDL surface
    renderer
        .draw("We can draw very simple text.")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 2, 0, 0))
        .unwrap();

    // Higher-range Unicode characters also work.
    renderer
        .draw("全角文字も対応しています。")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 20, 0, 0))
        .unwrap();

    // Text can be scaled by integer multiples
    renderer.scale = 6;
    renderer
        .draw("BIG text")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 80, 0, 0))
        .unwrap();

    renderer.scale = 1;
    renderer.fg_color = Color::RGB(255, 255, 255);
    renderer.bg_color = Color::RGB(0, 0, 0);
    renderer
        .draw("The background colour can also be changed.")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 180, 0, 0))
        .unwrap();

    /*
     * Rainbow text
     */
    let nc = colours.len();
    // Make text background transparent for overlapping
    renderer.bg_color = Color::RGBA(255, 255, 255, 0);
    renderer.scale = 3;
    // Cycle through colours
    for (i, colour) in colours[iter_num % nc..nc]
        .into_iter()
        .chain(colours[0..iter_num % nc].into_iter())
        .enumerate()
    {
        // Text colour can be changed per-draw operation
        renderer.fg_color = **colour;
        renderer
            .draw("Rainbow text")
            .unwrap()
            .blit(
                None,
                &mut screen,
                Rect::new(
                    2 + nc as i32 - i as i32,
                    38 + nc as i32 - i as i32,
                    0,
                    0,
                ),
            ).unwrap();
    }

    /*
     * Emoji demo - if Unicode plane 1 (SMP) support has been enabled. Also
     * demonstrates what happens if an unknown character is found in the string.
     */
    renderer.fg_color = Color::RGB(0, 0, 0);
    renderer.scale = 1;
    match renderer.draw(
        "🇪🇲🇴🇯🇮 are supported 🔥, as the plane-1 cargo feature is enabled 👌😂🤔") {
        Ok(surf) => surf.blit(None, &mut screen, Rect::new(2, 200, 0, 0)),
        Err(_) => renderer
            .draw("Enable emoji support by running the demo with \
                   `--features plane-1`")
            .unwrap()
            .blit(None, &mut screen, Rect::new(2, 200, 0, 0))
    }.unwrap();

    /*
     * Text formatting demos (bold/italic)
     */
    // Bold text
    renderer.bold = true;
    renderer
        .draw("Text can be made bold")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 220, 0, 0))
        .unwrap();

    // Italic text
    renderer.bold = false;
    renderer.italic = true;
    renderer
        .draw("...or Italicised")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 240, 0, 0))
        .unwrap();

    // Bold & Italicised
    renderer.bold = true;
    renderer
        .draw("...or both at the same time")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 260, 0, 0))
        .unwrap();

    // Italicised and scaled
    renderer.bold = false;
    renderer.scale = 3;
    renderer.bg_color = Color::RGB(255, 255, 0);
    renderer
        .draw("Formatting scales, too")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 280, 0, 0))
        .unwrap();

    /*
     * Combining text demo (this is what you have to do if you don't use the
     * `FormattedRenderer`)
     */
    renderer.italic = false;
    renderer.scale = 1;
    renderer.bg_color = Color::RGB(255, 0, 255);
    renderer.fg_color = Color::RGB(255, 255, 0);
    renderer
    //.draw("You _can_ use whatever colours you want, but just not this. Never this.")
        .draw("You ")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 330, 0, 0))
        .unwrap();

    let you_size = renderer.measure_width("You ").unwrap();

    renderer.italic = true;
    renderer
        .draw("can")
        .unwrap()
        .blit(
            None,
            &mut screen,
            Rect::new((2 + you_size) as i32, 330, 0, 0),
        ).unwrap();

    let can_size = renderer.measure_width("can").unwrap();

    renderer.italic = false;
    renderer
        .draw(" use whatever colours you want, just not these. Never these.")
        .unwrap()
        .blit(
            None,
            &mut screen,
            Rect::new((2 + you_size + can_size) as i32, 330, 0, 0),
        ).unwrap();

    /*
     * Very simple FormattedRenderer demo. Unsafe only because we use a global
     * variable to demonstrate updating variable values
     */
    unsafe {
        match G_FMT_RENDERER {
            None => {
                // Initialise format string first time round
                let mut renderer =
                    FormattedRenderer::new(Color::RGBA(0, 0, 0, 0));

                renderer.add_text(
                    "Text draw time: ",
                    Color::RGB(0, 0, 0),
                    false,
                    false,
                );
                renderer.add_var("frametime", Color::RGB(0, 0, 0), true, false);
                renderer.add_text("ms", Color::RGB(0, 0, 0), true, false);

                G_FMT_RENDERER = Some(renderer);
            }

            Some(ref mut renderer) => {
                // Update variable value on subsequent runs
                renderer.set_var(
                    "frametime",
                    &last_time.subsec_millis().to_string(),
                );

                // Render formatted string
                renderer
                    .draw()
                    .unwrap()
                    .blit(
                        None,
                        &mut screen,
                        Rect::new(
                            798 - renderer.measure_width().unwrap() as i32,
                            600 - 20,
                            0,
                            0,
                        ),
                    ).unwrap();
            }
        }
    };

    // Hand the finished surface back to the render loop for copying to screen
    screen
}

/*
 * SDL boilerplate (not really relevant for the library demo):
 */
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

fn main() {
    // SDL initialisers
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create window
    let window = video_subsystem
        .window("sdl2-unifont demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create and clear window canvas
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Window stays black whilst font decompression / parsing occurs
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Event / render loop
    let mut iter_num: usize = 0;
    let mut start: Instant;
    let mut draw_time: Duration = Duration::new(0, 0);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Draw demo
        start = Instant::now();
        let result_surf = draw_demo(iter_num, draw_time);
        draw_time = Instant::now().duration_since(start);

        let demo_tex = texture_creator
            .create_texture_from_surface(result_surf)
            .unwrap();
        canvas.copy(&demo_tex, None, None).unwrap();

        canvas.present();

        /* Uncomment to write unique frames to directory
        use sdl2::pixels::PixelFormatEnum;
        use sdl2::surface::Surface;
        use std::path::Path;
        if iter_num < colours.len() {
            let mut pixels =
                canvas.read_pixels(None, PixelFormatEnum::RGB888).unwrap();
            let surf = Surface::from_data(
                &mut pixels,
                800,
                600,
                3200,
                PixelFormatEnum::RGB888,
            ).unwrap();
            surf.save_bmp(
                Path::new("[INSERT DIRECTORY PATH HERE]")
                    .join(format!("0{}", iter_num))
                    .with_extension("bmp"),
            ).unwrap();
        }
        */

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        iter_num += 1;
    }
}
