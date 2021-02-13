use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use bit_field::BitField;

use std::collections::HashMap;
use std::slice::IterMut;

use unifont;

/// Number of vertical pixels in each Unifont character.
const UNIFONT_HEIGHT: u32 = 16;

/// Storage class for rendering settings.
pub struct SurfaceRenderer {
    /// The colour to use to draw text.
    pub fg_color: Color,
    /// The foreground colour supplied to the constructor
    fg_orig: Color,
    /// The colour to use to fill the surface before drawing text.
    pub bg_color: Color,
    /// The background colour supplied to the constructor
    bg_orig: Color,

    /// Integer scale multiplier, since Unifont is a raster font.
    pub scale: u32,
    /// Whether or not to make text bold. Uses XTerm-style bolding, where the
    /// text is just drawn twice on the x-axis, one pixel apart.
    pub bold: bool,
    /// Whether or not to make text italicised. Simply shifts pixels to the
    /// right by one additional pixel, every two vertical pixels.
    pub italic: bool,
}

impl SurfaceRenderer {
    /// Creates a new Unifont renderer which renders text to new SDL surfaces.
    pub fn new(fg_color: Color, bg_color: Color) -> SurfaceRenderer {
        SurfaceRenderer {
            fg_color,
            fg_orig: fg_color,
            bg_color,
            bg_orig: bg_color,
            scale: 1,
            bold: false,
            italic: false,
        }
    }

    /// Returns the renderer to the state it was in when it was first created
    /// (i.e. the foreground and background colours are reset to the values
    /// given to the constructor, and all other fields are reset).
    pub fn reset(&mut self) {
        self.fg_color = self.fg_orig;
        self.bg_color = self.bg_orig;
        self.scale = 1;
        self.bold = false;
        self.italic = false;
    }

    /// Draws the supplied text to a new surface, which has been sized to fit
    /// the text exactly, using the renderer's style settings. Returns an `Err`
    /// result if a character was found which is not in the font, or the font
    /// could not be initialised.
    pub fn draw(&self, text: &str) -> Result<Surface, String> {
        // Create new surface sized to text
        let width = self.measure_width(text)?;
        let mut surf = Surface::new(
            width,
            UNIFONT_HEIGHT * self.scale,
            PixelFormatEnum::RGBA8888,
        )?;

        // Fill surface with background color
        surf.fill_rect(None, self.bg_color)?;

        // Obtain raw surface data reference, then draw characters of string
        // through `draw_raw`
        if surf.must_lock() {
            surf.with_lock_mut(|px: &mut [u8]| self.draw_raw(px, &width, text))?
        } else {
            self.draw_raw(surf.without_lock_mut().unwrap(), &width, text)?
        }

        Ok(surf)
    }

    /// Sums the width of each character in the supplied text, and multiples the
    /// sum by the renderer's integer scale factor. Takes into consideration
    /// formatting options' effects on text width.
    pub fn measure_width(&self, text: &str) -> Result<u32, String> {
        let mut basic_width = self.scale * count_char_width(text)?;

        if self.bold {
            basic_width += self.scale;
        }
        if self.italic {
            basic_width += 8 * self.scale;
        }

        Ok(basic_width)
    }

    /// May in the future take into consideration newlines and other formatting.
    /// For now, it just returns `16 * scale`, thus, the result of this method
    /// can always be safely `unwrap()`ped.
    pub fn measure_height(&self, _text: &str) -> Result<u32, String> {
        Ok(self.scale * UNIFONT_HEIGHT)
    }

    /// Takes an array of pixels and draws the supplied text to it, using the
    /// specified render options. This function always assumes RGBA8888 pixel
    /// formatting.
    fn draw_raw(
        &self,
        pixels: &mut [u8],
        surf_width: &u32,
        text: &str,
    ) -> Result<(), String> {
        let unifont = get_unifont()?;

        // Start position of next character
        let mut x_offset = 0;

        let iter = text.chars();
        for c in iter {
            // Retrieve character description from hashmap
            let font_char = match unifont.get(&(c as u32)) {
                None => return Err(gen_missing_char_str(&c)),
                Some(font_char) => font_char,
            };

            // Draw rows of character bitmap
            for row in 0..UNIFONT_HEIGHT as usize {
                // Draw each pixel for a row
                for col in (0..font_char.width as usize).rev() {
                    if font_char.bitmap[row].get_bit(col) {
                        // Double character on x axis if we're bolding
                        for x in if self.bold {
                            0..self.scale * 2
                        } else {
                            0..self.scale
                        } {
                            for y in 0..self.scale {
                                // Calculate the byte position of the pixel
                                // (this thing is a mess, to be honest)
                                let px_base = (4
                                    * surf_width
                                    * (row as u32 * self.scale + y)
                                    + 4 * x_offset
                                    + 4 * (font_char.width as u32 * self.scale
                                        - col as u32 * self.scale
                                        - self.scale)
                                    + 4 * x)
                                    as usize;

                                // Insert fg colour into the current pixel
                                // TODO assumes little endian
                                pixels[px_base + 3] = self.fg_color.r;
                                pixels[px_base + 2] = self.fg_color.g;
                                pixels[px_base + 1] = self.fg_color.b;
                                pixels[px_base] = self.fg_color.a;
                            }
                        }
                    }
                }
            }

            // Shift next character
            x_offset += self.scale * font_char.width as u32;
        }

        // Italicise text
        if self.italic {
            let mut offset = (UNIFONT_HEIGHT * self.scale) / 2;
            for row in 0..UNIFONT_HEIGHT * self.scale as u32 {
                let row_offset = 4 * row * surf_width;
                // Shift bytes forward
                for i in
                    (row_offset..row_offset + 4 * (surf_width - offset)).rev()
                {
                    pixels[(i + 4 * offset) as usize] = pixels[i as usize];
                }

                // Clear space behind first character
                for i in (row_offset as usize
                    ..(row_offset + offset * 4) as usize)
                    .step_by(4)
                {
                    // TODO assumes little endian
                    pixels[i + 3] = self.bg_color.r;
                    pixels[i + 2] = self.bg_color.g;
                    pixels[i + 1] = self.bg_color.b;
                    pixels[i] = self.bg_color.a;
                }

                if row % 2 == 1 {
                    offset -= 1;
                }
            }
        }

        Ok(())
    }
}

/// Advanced renderer with additional capabilities.
pub struct FormattedRenderer {
    /// Stores variables and string literals. The boolean value is set to `true`
    /// for literals, and `false` for variable name references.
    text: Vec<(bool, String)>,
    /// Stores a `SurfaceRenderer` for each entry in the `text` vector.
    renderers: Vec<SurfaceRenderer>,
    /// Maps variable names to values.
    variables: HashMap<String, String>,
    /// The colour to use behind all text.
    bg_color: Color,
    /// The scale to use for all text.
    scale: u32,
}

impl FormattedRenderer {
    /// Creates a new blank `FormattedRenderer`. All segments use the same
    /// background colour.
    pub fn new(bg_color: Color) -> FormattedRenderer {
        FormattedRenderer {
            text: Vec::new(),
            renderers: Vec::new(),
            variables: HashMap::new(),
            bg_color,
            scale: 1,
        }
    }

    /// Adds a string literal, which cannot be modified once added (i.e. you'll
    /// need to create a new `FormattedRenderer` instead).
    pub fn add_text(
        &mut self,
        text: &str,
        color: Color,
        bold: bool,
        italic: bool,
    ) {
        self.text.push((true, text.to_string()));
        let mut renderer = SurfaceRenderer::new(color, self.bg_color);
        renderer.bold = bold;
        renderer.italic = italic;
        renderer.scale = self.scale;
        self.renderers.push(renderer);
    }

    /// Adds a named variable, which can have its value and changed after
    /// creation.
    pub fn add_var(
        &mut self,
        name: &str,
        color: Color,
        bold: bool,
        italic: bool,
    ) {
        self.text.push((false, name.to_string()));
        let mut renderer = SurfaceRenderer::new(color, self.bg_color);
        renderer.bold = bold;
        renderer.italic = italic;
        renderer.scale = self.scale;
        self.renderers.push(renderer);
        self.variables
            .insert(name.to_string(), "#UNDEFINED".to_string());
    }

    /// Sets or modifies the value of an already added variable. If the variable
    /// referenced does not exist, nothing happens.
    pub fn set_var(&mut self, name: &str, value: &str) {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value.to_string());
        }
    }

    /// Sets the background color of each component of the formatted output.
    pub fn set_bg_color(&mut self, bg_color: Color) {
        self.bg_color = bg_color;
        for renderer in self.renderers.iter_mut() {
            renderer.bg_color = bg_color;
        }
    }

    /// Returns the default background colour for each rendered section (unless
    /// it's been changed by modifying a renderer's background colour through
    /// the `iter_mut` method).
    pub fn get_bg_color(&self) -> Color {
        return self.bg_color;
    }

    /// Sets the scale of each component in the formatted output.
    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale;
        for renderer in self.renderers.iter_mut() {
            renderer.scale = scale;
        }
    }

    /// Gets the current scale factor used for draw operations.
    pub fn get_scale(&self) -> u32 {
        return self.scale;
    }

    /// Returns an iterator over each renderer, which allows the renderers'
    /// settings to be modified.
    pub fn iter_mut(&mut self) -> IterMut<SurfaceRenderer> {
        self.renderers.iter_mut()
    }

    /// Sequentially draws each literal and variable, using its associated
    /// renderer, and linearly appends the output surfaces.
    pub fn draw<'a>(&self) -> Result<Surface<'a>, String> {
        // Preflight width sum
        let width = self.measure_width()?;

        // Create output surface
        let mut surf = Surface::new(
            width,
            UNIFONT_HEIGHT * self.scale,
            PixelFormatEnum::RGBA8888,
        )?;

        // Draw text
        let mut offset: u32 = 0;
        for (text, renderer) in
            (&self.text).into_iter().zip((&self.renderers).into_iter())
        {
            let text = if text.0 {
                &text.1
            } else {
                match self.variables.get(&text.1) {
                    Some(val) => val,
                    None => return Err("Undefined variable used".to_string()),
                }
            };

            renderer.draw(text)?.blit(
                None,
                &mut surf,
                Rect::new(offset as i32, 0, 0, 0),
            )?;
            offset += renderer.measure_width(text)?;
        }

        Ok(surf)
    }

    /// Measures the width of all of the contained text, including variable
    /// values, taking into consideration the formatting of each section.
    pub fn measure_width(&self) -> Result<u32, String> {
        let mut width = 0;
        for (text, renderer) in
            (&self.text).into_iter().zip((&self.renderers).into_iter())
        {
            if text.0 {
                width += renderer.measure_width(&text.1)?;
            } else {
                match self.variables.get(&text.1) {
                    Some(val) => width += renderer.measure_width(val)?,
                    None => return Err("Undefined variable used".to_string()),
                }
            }
        }

        Ok(width)
    }

    /// Returns the height of all content in the formatted string.
    pub fn measure_height(&self) -> Result<u32, String> {
        Ok(self.scale * UNIFONT_HEIGHT)
    }
}

impl IntoIterator for FormattedRenderer {
    type Item = SurfaceRenderer;
    type IntoIter = ::std::vec::IntoIter<SurfaceRenderer>;

    fn into_iter(self) -> Self::IntoIter {
        self.renderers.into_iter()
    }
}

/// Maps `unifont`'s `Result` error type to ours, so that the `?` operator
/// can be utilised.
fn get_unifont<'a>() -> Result<&'a unifont::FontChars, String> {
    match unifont::get_unifont() {
        Ok(unifont) => Ok(unifont),
        Err(_) => {
            return Err("Failed to initialise embedded Unifont".to_string())
        }
    }
}

/// Finds the rendered width of a string, taking into consideration whether each
/// character is half-width (8px) or full-width (16px). Returns an error result
/// if a character is not found in the font (i.e. the feature to include it was
/// probably not enabled).
fn count_char_width(text: &str) -> Result<u32, String> {
    let unifont = get_unifont()?;

    let mut width_sum: u32 = 0;
    let iter = text.chars();

    for c in iter {
        match unifont.get(&(c as u32)) {
            None => return Err(gen_missing_char_str(&c)),
            Some(fc) => width_sum += fc.width as u32,
        }
    }

    Ok(width_sum)
}

fn gen_missing_char_str(c: &char) -> String {
    format!(
        "Embedded Unifont does not contain {} (code point: 0x{:x})",
        c, *c as u32
    )
}
