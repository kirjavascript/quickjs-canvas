use css_color::Rgba;
use sdl2::pixels::Color;

pub fn parse(color: &str) -> Option<Color> {
    let color: Rgba = color.parse().ok()?;
    Some(
        Color::RGBA(
            (color.red * 256.) as _,
            (color.green * 256.) as _,
            (color.blue * 256.) as _,
            (color.alpha * 256.) as _,
        )
    )
}

pub fn web_format(color: &Color) -> String {
    if color.a == 255 {
        // hex
        format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
    } else {
        // rgba
        format!(
            "rgba({}, {}, {}, {})",
            color.r,
            color.g,
            color.b,
            (color.a as f64 / 256.),
        )
    }
}
