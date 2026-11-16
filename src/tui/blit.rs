use ratatui::{buffer::Buffer, layout::Rect, style::Color};

pub(crate) fn blit_by_scale(
    c: &[&str],
    (left, bottom): (u16, u16),
    scale: u16,
    area: Rect,
    buf: &mut Buffer,
    color: Color,
) {
    let mut bottom = bottom;
    for row in c.iter() {
        let mut left = left;

        for c in row.chars() {
            match c {
                ' ' => (),
                c => {
                    for i_ in left..(left + scale).min(area.width - 1) {
                        for j_ in bottom..(bottom + scale).min(area.height - 1) {
                            buf[(i_, j_)].set_char(c).set_fg(color);
                        }
                    }
                }
            }

            left += scale;
        }
        bottom += scale;
    }
}
