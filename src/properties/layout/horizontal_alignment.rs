use super::Margin;
use crate::{structs::Spacer, widget::Template};

/// Used to horizontal align a widget.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HorizontalAlignment {
    /// Align left.
    Left,

    /// Align center.
    Center,

    /// Align right.
    Right,

    /// Stretch to available width.
    Stretch,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Stretch
    }
}

impl HorizontalAlignment {
    /// Calculates the x position of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_x(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Right => available_width - width - margin.right(),
            HorizontalAlignment::Center => (available_width - width) / 2.0,
            _ => margin.left(),
        }
    }

    /// Calculates the width of the widget depending on the available width, the goal width
    /// margin and horizontal alignment.
    pub fn align_width(&self, available_width: f64, width: f64, margin: Margin) -> f64 {
        match self {
            HorizontalAlignment::Stretch => available_width - margin.left() - margin.right(),
            _ => width,
        }
    }
}

impl From<&str> for HorizontalAlignment {
    fn from(t: &str) -> Self {
        match t {
            "Right" | "right" => {
                HorizontalAlignment::Right
            },
            "Center" | "center" => {
                HorizontalAlignment::Center
            },
            "Left" | "left" => {
                HorizontalAlignment::Left
            },
            _ => HorizontalAlignment::Stretch
        }
    }
}

pub trait HorizontalAlignmentProperty: Sized + From<Template> + Into<Template> {
    fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
        Self::from(transform(self.into()))
    }

    fn horizontal_alignment<H: Into<HorizontalAlignment>>(self, horizontal_alignment: H) -> Self {
        self.template(|template| {
            template.property(horizontal_alignment.into())
        })
    }
}