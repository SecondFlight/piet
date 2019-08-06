//! Gradients.

use piet::kurbo::{Point, Rect, Vec2};

use piet::{
    Color, Error, GradientStop, RawGradient, RawLinearGradient, RawRadialGradient, RenderContext,
};

pub fn draw<R: RenderContext>(rc: &mut R) -> Result<(), Error> {
    rc.clear(Color::WHITE);
    let stops = vec![
        GradientStop {
            pos: 0.0,
            color: Color::rgb24(0xff_ff_ff),
        },
        GradientStop {
            pos: 1.0,
            color: Color::rgb24(0x00_00_00),
        },
    ];
    let gradient = rc.gradient(RawGradient::Radial(RawRadialGradient {
        center: Point::new(30.0, 30.0),
        origin_offset: Vec2::new(10.0, 10.0),
        radius: 30.0,
        stops,
    }))?;
    rc.fill(Rect::new(0.0, 0.0, 60.0, 60.0), &gradient);
    let stops2 = vec![
        GradientStop {
            pos: 0.0,
            color: Color::rgb24(0xff_ff_ff),
        },
        GradientStop {
            pos: 1.0,
            color: Color::rgb24(0x00_00_00),
        },
    ];
    let gradient2 = rc.gradient(RawGradient::Linear(RawLinearGradient {
        start: Point::new(0.0, 0.0),
        end: Point::new(60.0, 0.0),
        stops: stops2,
    }))?;
    rc.fill(Rect::new(0.0, 80.0, 60.0, 100.0), &gradient2);
    Ok(())
}
