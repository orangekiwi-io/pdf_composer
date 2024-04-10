# PDF Composer
> **PDF generation from Yaml Front Matter documents for Rust**

## Overview
This crate creates a PDF document from YAML Front Matter source documents. The YAML values can be used for PDF Dictionary entries or used to replace placeholder references within the Markdown section of the YAML Front Matter document.

## Features

### PDF output destination
Generated PDFs are saved to an output destination directory relative to your project root.

For example, `my_pdf_instance.set_output_directory("output_pdfs")`. If no output destination is set, then PDFs will be saved in a directory called `pdf_composer_pdfs`. This stops the root of your project being littered with generated PDFs. Nice and tidy.

### PDF versions
Currently only the latest two versions of the PDF specifications are supported (and encouraged), namely versions `1.7` and `2.0`.

For example, `my_pdf_instance.set_pdf_version(PDFVersion::V2_0)`. If no PDF version is set, then version `1.7` is used by default.

**NOTE:** This is not to be confused with the version/edition of the PDF itself. That is something for you to decide (and maybe set as a YAML value for inclusion as a PDF Dictionary entry, see below for more information).

To set the version see the table showing the corresponding enum with version number.

| enum | PDF Version |
| --- | --- |
| PDFVersion::V1_7 | 1.7 |
| PDFVersion::V2_0 | 2.0 |

### Paper sizes
List of supported paper sizes. For example, `my_pdf_instance.set_paper_size(PaperSize::A5)`. If no paper size is set, the paper size defaults to `A4`.

#### ISO 216 A series paper sizes

| Size | Width x Height (mm) | Width x Height (inches) |
|---|---|---|
| A0  | 841mm x 1189mm | 33.1" x 46.8" |
| A1  | 594mm x 841mm  | 23.4" x 33.1" |
| A2  | 420mm x 594mm  | 16.5" x 23.4" |
| A3  | 297mm x 420mm  | 11.7" x 16.5" |
| A4  | 210mm x 297mm  | 8.3" x 11.7"  |
| A5  | 148mm x 210mm  | 5.8" x 8.3"   |
| A6  | 105mm x 148mm  | 4.1" x 5.8"   |
| A7  | 74mm x 105mm   | 2.9" x 4.1"   |
| A8  | 52mm x 74mm    | 2.0" x 2.9"   |
| A9  | 37mm x 52mm    | 1.5" x 2.0"   |
| A10 | 26mm x 37mm    | 1.0" x 1.5"   |

#### ISO 216 B series paper sizes

| Size | Width x Height (mm) | Width x Height (inches) |
|---|---|---|
| B0  | 1000mm x 1414mm | 39.4" x 55.7" |
| B1  | 707mm x 1000mm  | 27.8" x 39.4" |
| B2  | 500mm x 707mm   | 19.7" x 27.8" |
| B3  | 353mm x 500mm   | 13.9" x 19.7" |
| B4  | 250mm x 353mm   | 9.8" x 13.9"  |
| B5  | 176mm x 250mm   | 6.9" x 9.8"   |
| B6  | 125mm x 176mm   | 4.9" x 6.9"   |
| B7  | 88mm x 125mm    | 3.5" x 4.9"   |
| B8  | 62mm x 88mm     | 2.4" x 3.5"   |
| B9  | 44mm x 62mm     | 1.7" x 2.4"   |
| B10 | 31mm x 44mm     | 1.2" x 1.7"   |

#### US paper sizes

| Size         | Width x Height (mm) | Width x Height (inches) |
|---|---|---|
| Half Letter  | 140mm x 216mm | 5.5" x 8.5" |
| Letter       | 216mm x 279mm | 8.5" x 11"  |
| Legal        | 216mm x 356mm | 8.5" x 14"  |
| Junior Legal | 203mm x 127mm | 8" x 5"     |
| Ledger       | 432mm x 279mm | 17" x 11"   |
| Tabloid      | 279mm x 432mm | 11" x 17"   |

### JIS paper sizes

| Size | Width x Height (mm) | Width x Height (in) |
|---|---|---|
| B0  | 1030 x 1456 mm | 40.6 x 57.3 in |
| B1  | 728 x 1030 mm  | 28.7 x 40.6 in |
| B2  | 515 x 728 mm   | 20.3 x 28.7 in |
| B3  | 364 x 515 mm   | 14.3 x 20.3 in |
| B4  | 257 x 364 mm   | 10.1 x 14.3 in |
| B5  | 182 x 257 mm   | 7.2 x 10.1 in  |
| B6  | 128 x 182 mm   | 5.0 x 7.2 in   |
| B7  | 91 x 128 mm    | 3.6 x 5.0 in   |
| B8  | 64 x 91 mm     | 2.5 x 3.6 in   |
| B9  | 45 x 64 mm     | 1.8 x 2.5 in   |
| B10 | 32 x 45 mm     | 1.3 x 1.8 in   |

### Paper orientation

This allows the PDF document pages to be saved as landscape or portrait orientation. For example, `my_pdf_instance.set_orientation(PaperOrientation::Landscape)`. If not orientation is set, the the default is `Portrait`.

### Page margins

The unit used for margins is millimeters (mm). If not margins set, the default margin size is `10`.

Set the page margins using CSS shorthand notation (top right bottom left). This means:

One value set, then all four margins will get that value. For example, `my_pdf_instance.set_margins("20")` will result in the following:
* margin-top: 20,
* margin-right: 20,
* margin-bottom: 20,
* margin-left: 20

Two values set, then the top and bottom margins will get the first value, the right and left margins will get the second value. For example, `my_pdf_instance.set_margins("20 15")` will result in the following:
* margin-top: 20,
* margin-right: 15,
* margin-bottom: 20,
* margin-left: 15

Three values set, then the top margin will get the first value, the right and left margins will get the second value, the bottom margin will the the third value. For example, `my_pdf_instance.set_margins("20 15 30")` will result in the following:
* margin-top: 20,
* margin-right: 15,
* margin-bottom: 30,
* margin-left: 15

Four values set, then the top margin will get the first value, the right will get the second value, the bottom margin will the the third value, the left will get the forth value. For example, `my_pdf_instance.set_margins("20 15 30 5")` will result in the following:
* margin-top: 20,
* margin-right: 15,
* margin-bottom: 30,
* margin-left: 5

If any other values (or non-integer number, letters, characters etc) are set, the the default value of `10` will be set for each margin.

### Page font

One of 14 standard fonts can be used for PDF documents. These are:

| Enum | Font name | Font weight | Font style |
|---|---|---|---|
| Courier              | Courier       | Normal | Normal  |
| CourierBold          | Courier       | Bold   | Normal  |
| CourierBoldOblique   | Courier       | Bold   | Oblique |
| CourierOblique       | Courier       | Normal | Oblique |
| Helvetica            | Helvetica     | Normal | Normal  |
| HelveticaBold        | Helvetica     | Bold   | Normal  |
| HelveticaBoldOblique | Helvetica     | Bold   | Oblique |
| HelveticaOblique     | Helvetica     | Normal | Oblique |
| Symbol               | Symbol        | Normal | Normal  |
| TimesBold            | Times         | Bold   | Normal  |
| TimesBoldItalic      | Times         | Bold   | Italic  |
| TimesItalic          | Times         | Normal | Italic  |
| TimesRoman           | Times         | Normal | Normal  |
| ZapfDingbats         | Zapf Dingbats | Normal | Normal  |

For example, `my_pdf_instance.set_font(FontsStandard::TimesRoman)`. If no font is set, the font defaults to `Helvetica`.

## PDF Dictionary entries

PDF Dictionary entries are those Name and Value pairs you can see if you selcted "Document Properties" within a PDF Reader on a PDF document. Dictionary entries are case sensitive, with a few reserved names.

### Reserved Dictionary Entries
* Title
* Author
* Subject
* Keywords

These **must** be capitalised. **PDF Composer** automatically captialises the reserved named ones only. All others will be left as entered.

In **PDF Composer** The Title entry is a special case. As part of the PDF generation process, the `title` value from the YAML document is automatically inserted into the `<title>` tag in the HTML templates used. As a result, the Dictionary entry is populated. If no YAML value is found, then the filename of the source file will be used instead.

Empty entries are **not** allowed. If no corresponding YAML entry can be found, then an empty entry will not be added to the PDF document.

For example, if you want to set a Dictionary entry called `Language` and you set it to a YAML entry that does not exist in the YAML document, **PDF Composer** will not create an empty entry.

### Example for setting a Dictionary entry
```rust
PDFDocInfoEntry {
    doc_info_entry: "Subject",
    yaml_entry: "description",
}
```

`doc_info_entry` is the PDF Dictionary entry.

`yaml_entry` is the YAML value that will be assigned to the Dictionary entry.

## YAML Markdown placeholder substitution

It is possible to simple substitution within the markdown section of the YAML document. This is possible by using `{{my_yaml_value}}` within the markdown section.

### Example
```yaml
---

# Front Matter (YAML)

author: "Richard"

---

The author of this document is {{author}}.

```

The result will be: `The author of this document is Richard.`

If the YAML value cannot be found, then the substitution placeholder will remain in the generated output.

### Example
```yaml
---

# Front Matter (YAML)

author: "Richard"

---

The author of this document is {{name}}.

```

The result will be: `The author of this document is {{name}}.`

## Example usage

Assuming you have Rust up and running (tested with rust verion `1.76+`) and you have run `cargo add pdf_composer` to install the **PDF Composer** crate, then you can begin.

```rust
use pdf_composer::{FontsStandard, PaperOrientation, PaperSize, PDFComposer, PDFDocInfoEntry, PDFVersion};

// Create a new PDFComposer instance
let mut my_pdf_instance = PDFComposer::new();

// Add some paths. Relative paths
let paths = vec![
    PathBuf::from("source_mds/file_01.md"),
    PathBuf::from("source_mds/file_02.md")
];
my_pdf_instance.add_source_files(paths);

// PDF version (not the version of the document, but the Adobe (formerly) PDF format version)
my_pdf_instance.set_pdf_version(PDFVersion::V2_0);

// Output directory for the generated PDFs
my_pdf_instance.set_output_directory("example_pdfs");

// Set the paper size
my_pdf_instance.set_paper_size(PaperSize::A5);

// Set the paper orientation
my_pdf_instance.set_orientation(PaperOrientation::Landscape);

// Set the page margins
my_pdf_instance.set_margins("20");

// Set font
my_pdf_instance.set_font(FontsStandard::TimesRoman);

// Metadata for the PDFs
let author_entry = PDFDocInfoEntry {
    doc_info_entry: "Author",
    yaml_entry: "author",
};
let keywords_entry = PDFDocInfoEntry {
    doc_info_entry: "Keywords",
    yaml_entry: "keywords",
};
let subject_entry = PDFDocInfoEntry {
    doc_info_entry: "Subject",
    yaml_entry: "description",
};
let language_entry = PDFDocInfoEntry {
    doc_info_entry: "Language",
    yaml_entry: "language",
};
my_pdf_instance.set_doc_info_entry(author_entry);
my_pdf_instance.set_doc_info_entry(keywords_entry);
my_pdf_instance.set_doc_info_entry(subject_entry);
my_pdf_instance.set_doc_info_entry(language_entry);

// Generate the PDF(s)
my_pdf_instance.generate_pdfs();

```

## License

The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

- [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/)
- [MIT license](https://opensource.org/licenses/MIT)

## Future plans

Some ideas, but not limited to:
* Set paper sizes, orientation and margins
* HTML templates
* Pretty default print stylesheets
* Allow for direct String values to be used for PDF Dictionary entries without having to have a YAML value first
* Combine multiple YAML Front Matter documents into one generated PDF document
* Pagination and page numbers