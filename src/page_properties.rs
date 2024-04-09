/// Enum to represent different supported paper sizes
/// See the following for more information:
/// * A Series Paper Sizes (ISO 216): <https://www.papersizes.org/a-paper-sizes.htm>
/// * B Series Paper Sizes (ISO 216): <https://www.papersizes.org/b-paper-sizes.htm>
/// * US Paper Sizes: <https://www.papersizes.org/us-paper-sizes.htm>
/// * Japanese Paper Sizes: <https://www.papersizes.org/japanese-sizes.htm>
#[derive(Clone, Copy, Debug)]
pub enum PaperSize {
    /// A0
    A0,
    /// A1
    A1,
    /// A2
    A2,
    /// A3
    A3,
    /// A4
    A4,
    /// A5
    A5,
    /// A6
    A6,
    /// A7
    A7,
    /// A8
    A8,
    /// A9
    A9,
    /// A10
    A10,
    /// B0
    B0,
    /// B1,
    B1,
    /// B2,
    B2,
    /// B3,
    B3,
    /// B4,
    B4,
    /// B5,
    B5,
    /// B6,
    B6,
    /// B7,
    B7,
    /// B8,
    B8,
    /// B9,
    B9,
    /// B10,
    B10,
    /// Half Letter,
    HalfLetter,
    /// Letter,
    Letter,
    /// Legal,
    Legal,
    /// JuniorLegal,
    JuniorLegal,
    /// Ledger,
    Ledger,
    /// Tabloid,
    Tabloid,
    /// JIS B0,
    JISB0,
    /// JIS B1,
    JISB1,
    /// JIS B2,
    JISB2,
    /// JIS B3,
    JISB3,
    /// JIS B4,
    JISB4,
    /// JIS B5,
    JISB5,
    /// JIS B6,
    JISB6,
    /// JIS B7,
    JISB7,
    /// JIS B8,
    JISB8,
    /// JIS B9,
    JISB9,
    /// JIS B10,
    JISB10,
}

type PageUnit = f64;
pub trait ToDimensions {
    // fn to_dimensions(&self) -> PaperDimension;
    fn to_dimensions(&self) -> (PageUnit, PageUnit);
}

#[derive(Clone, Debug)]
pub struct PaperDimension {
    width: PageUnit,
    height: PageUnit,
}

impl ToDimensions for PaperSize {
    /// Implements the ToDimensions trait for PaperSize,
    /// converting enum variants to width and height sizes in millimeters (mm)
    fn to_dimensions(&self) -> (PageUnit, PageUnit) {
        // fn to_dimensions(&self) -> PaperDimension {
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
