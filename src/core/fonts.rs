/// Enum representing the standard 14 PostScript fonts available to use in PDF documents
#[derive(Clone, Copy, Debug)]
pub enum FontsStandard {
    /// Courier font
    Courier,
    /// Courier font bold
    CourierBold,
    /// Courier font bold and italic
    CourierBoldOblique,
    /// Courier font italic
    CourierOblique,
    /// Helvetica
    Helvetica,
    /// Helvetica font bold
    HelveticaBold,
    /// Helvetica font bold and italic
    HelveticaBoldOblique,
    /// Helvetica font italic
    HelveticaOblique,
    /// Symbol font
    Symbol,
    /// Times font bold
    TimesBold,
    /// Times font bold and italic
    TimesBoldItalic,
    /// Times font italic
    TimesItalic,
    /// Times Roman font
    TimesRoman,
    /// Zapf Dingbats font
    ZapfDingbats,
}

/// Trait to return the CSS name of the standard font passed in
pub(crate) trait GetCssName {
    fn get_css_name(&self) -> (String, String, String);
}

impl GetCssName for FontsStandard {
    fn get_css_name(&self) -> (String, String, String) {
        let font_name = match self {
            FontsStandard::Courier
            | FontsStandard::CourierBold
            | FontsStandard::CourierBoldOblique
            | FontsStandard::CourierOblique => "Courier, monospace",
            FontsStandard::Helvetica
            | FontsStandard::HelveticaBold
            | FontsStandard::HelveticaBoldOblique
            | FontsStandard::HelveticaOblique => "Helvetica, sans-serif",
            FontsStandard::Symbol => "Symbol",
            FontsStandard::TimesBold
            | FontsStandard::TimesBoldItalic
            | FontsStandard::TimesItalic
            | FontsStandard::TimesRoman => "'Times New Roman', Times, serif",
            FontsStandard::ZapfDingbats => "'Zapf Dingbats'",
        };

        let font_weight = match self {
            FontsStandard::CourierBold
            | FontsStandard::CourierBoldOblique
            | FontsStandard::HelveticaBold
            | FontsStandard::HelveticaBoldOblique
            | FontsStandard::TimesBold
            | FontsStandard::TimesBoldItalic => "bold",
            _ => "normal",
        };

        let font_style = match self {
            FontsStandard::CourierBoldOblique
            | FontsStandard::CourierOblique
            | FontsStandard::HelveticaBoldOblique
            | FontsStandard::HelveticaOblique
            | FontsStandard::TimesBoldItalic
            | FontsStandard::TimesItalic => "italic",
            _ => "normal",
        };

        (
            font_name.to_string(),
            font_weight.to_string(),
            font_style.to_string(),
        )
    }
}
