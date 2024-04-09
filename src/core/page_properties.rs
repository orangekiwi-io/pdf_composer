/// Enum representing paper orientation
#[derive(Clone, Copy, Debug)]
pub enum PaperOrientation {
    Landscape,
    Portrait,
}

/// Enum representing different paper sizes
/// See the following for more information:
/// * A Series Paper Sizes (ISO 216): <https://www.papersizes.org/a-paper-sizes.htm>
/// * B Series Paper Sizes (ISO 216): <https://www.papersizes.org/b-paper-sizes.htm>
/// * US Paper Sizes: <https://www.papersizes.org/us-paper-sizes.htm>
/// * Japanese Paper Sizes: <https://www.papersizes.org/japanese-sizes.htm>
/// Paper sizes from ISO As and Bs, US Letter and Japanese sizes
#[derive(Clone, Copy, Debug)]
pub enum PaperSize {
    /// A0 (ISO 216)
    A0,
    /// A1 (ISO 216)
    A1,
    /// A2 (ISO 216)
    A2,
    /// A3 (ISO 216)
    A3,
    /// A4 (ISO 216)
    A4,
    /// A5 (ISO 216)
    A5,
    /// A6 (ISO 216)
    A6,
    /// A7 (ISO 216)
    A7,
    /// A8 (ISO 216)
    A8,
    /// A9 (ISO 216)
    A9,
    /// A10 (ISO 216)
    A10,
    /// B0 (ISO 216)
    B0,
    /// B1 (ISO 216)
    B1,
    /// B2 (ISO 216)
    B2,
    /// B3 (ISO 216)
    B3,
    /// B4 (ISO 216)
    B4,
    /// B5 (ISO 216)
    B5,
    /// B6 (ISO 216)
    B6,
    /// B7 (ISO 216)
    B7,
    /// B8 (ISO 216)
    B8,
    /// B9 (ISO 216)
    B9,
    /// B10 (ISO 216)
    B10,
    /// US Half Letter
    HalfLetter,
    /// US Letter
    Letter,
    /// US Legal
    Legal,
    /// US Junior Legal
    JuniorLegal,
    /// US Ledger
    Ledger,
    /// US Tabloid
    Tabloid,
    /// Japanese JIS B0
    JISB0,
    /// Japanese JIS B1
    JISB1,
    /// Japanese JIS B2
    JISB2,
    /// Japanese JIS B3
    JISB3,
    /// Japanese JIS B4
    JISB4,
    /// Japanese JIS B5
    JISB5,
    /// Japanese JIS B6
    JISB6,
    /// Japanese JIS B7
    JISB7,
    /// Japanese JIS B8
    JISB8,
    /// Japanese JIS B9
    JISB9,
    /// Japanese JIS B10
    JISB10,
}

/// Type for the unit Headless Chrome prefers for setting page size.
/// Headless Chrome uses inches.
/// The type is set here in case the units change from `f64`
type PageUnit = f64;

/// Trait to extract the width and height of the paper size from the `PaperSize` chosen
pub (crate) trait ToDimensions {
    fn to_dimensions(&self) -> (PageUnit, PageUnit);
}

#[derive(Clone, Debug)]
pub struct PaperDimension {
    width: PageUnit,
    height: PageUnit,
}

impl ToDimensions for PaperSize {
    /// Implements the `to_dimensions` trait for `PaperSize`,
    /// converting enum variants to width and height sizes in inches (in)
    fn to_dimensions(&self) -> (PageUnit, PageUnit) {
        let output = match self {
            PaperSize::A0 => PaperDimension {
                width: 33.1,
                height: 46.8,
            },
            PaperSize::A1 => PaperDimension {
                width: 23.4,
                height: 33.1,
            },
            PaperSize::A2 => PaperDimension {
                width: 16.5,
                height: 23.4,
            },
            PaperSize::A3 => PaperDimension {
                width: 11.7,
                height: 16.5,
            },
            PaperSize::A4 => PaperDimension {
                width: 8.3,
                height: 11.7,
            },
            PaperSize::A5 => PaperDimension {
                width: 5.8,
                height: 8.3,
            },
            PaperSize::A6 => PaperDimension {
                width: 4.1,
                height: 5.8,
            },
            PaperSize::A7 => PaperDimension {
                width: 2.9,
                height: 4.1,
            },
            PaperSize::A8 => PaperDimension {
                width: 2.0,
                height: 2.9,
            },
            PaperSize::A9 => PaperDimension {
                width: 1.5,
                height: 2.0,
            },
            PaperSize::A10 => PaperDimension {
                width: 1.0,
                height: 1.5,
            },
            PaperSize::B0 => PaperDimension {
                width: 39.4,
                height: 55.7,
            },
            PaperSize::B1 => PaperDimension {
                width: 27.8,
                height: 39.4,
            },
            PaperSize::B2 => PaperDimension {
                width: 19.7,
                height: 27.8,
            },
            PaperSize::B3 => PaperDimension {
                width: 13.9,
                height: 19.7,
            },
            PaperSize::B4 => PaperDimension {
                width: 9.8,
                height: 13.9,
            },
            PaperSize::B5 => PaperDimension {
                width: 6.9,
                height: 9.8,
            },
            PaperSize::B6 => PaperDimension {
                width: 4.9,
                height: 6.9,
            },
            PaperSize::B7 => PaperDimension {
                width: 3.5,
                height: 4.9,
            },
            PaperSize::B8 => PaperDimension {
                width: 2.4,
                height: 3.5,
            },
            PaperSize::B9 => PaperDimension {
                width: 1.7,
                height: 2.4,
            },
            PaperSize::B10 => PaperDimension {
                width: 1.2,
                height: 1.7,
            },
            PaperSize::HalfLetter => PaperDimension {
                width: 5.5,
                height: 8.5,
            },
            PaperSize::Letter => PaperDimension {
                width: 8.5,
                height: 11.0,
            },
            PaperSize::Legal => PaperDimension {
                width: 8.5,
                height: 14.0,
            },
            PaperSize::JuniorLegal => PaperDimension {
                width: 8.0,
                height: 5.0,
            },
            PaperSize::Ledger => PaperDimension {
                width: 17.0,
                height: 11.0,
            },
            PaperSize::Tabloid => PaperDimension {
                width: 11.0,
                height: 17.0,
            },
            PaperSize::JISB0 => PaperDimension {
                width: 40.6,
                height: 57.3,
            },
            PaperSize::JISB1 => PaperDimension {
                width: 28.7,
                height: 40.6,
            },
            PaperSize::JISB2 => PaperDimension {
                width: 20.3,
                height: 28.7,
            },
            PaperSize::JISB3 => PaperDimension {
                width: 14.3,
                height: 20.3,
            },
            PaperSize::JISB4 => PaperDimension {
                width: 10.1,
                height: 14.3,
            },
            PaperSize::JISB5 => PaperDimension {
                width: 7.2,
                height: 10.1,
            },
            PaperSize::JISB6 => PaperDimension {
                width: 5.0,
                height: 7.2,
            },
            PaperSize::JISB7 => PaperDimension {
                width: 3.6,
                height: 5.0,
            },
            PaperSize::JISB8 => PaperDimension {
                width: 2.5,
                height: 3.6,
            },
            PaperSize::JISB9 => PaperDimension {
                width: 1.8,
                height: 2.5,
            },
            PaperSize::JISB10 => PaperDimension {
                width: 1.3,
                height: 1.8,
            },
        };

        (output.width, output.height)
    }
}
