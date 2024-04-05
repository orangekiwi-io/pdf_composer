# PDF Composer
> **PDF generation from Yaml Front Matter documents for Rust**

## Overview
This crate creates a PDF document from YAML Front Matter source documents. The YAML values can be used for PDF Dictionary entries or used to replace placeholder references within the Markdown section of the YAML Front Matter document.

## Features

### PDF output destination
Generated PDFs are saved to an output destination directory relative to your project root.

If no output destination is set, then PDFs will be saved in a directory called `pdf_composer_pdfs`. This stops the root of your project being littered with generated PDFs. Nice and tidy.

### PDF versions
Currently only the latest two versions of the PDF specifications are supported (and encouraged), namely versions 1.7 and 2.0.

If not PDF version is set, then version 1.7 is used by default.

**NOTE:** This is not to be confused with the version/edition of the PDF itself. That is something for you to decide (and maybe set as a YAML value for inclusion as a PDF Dictionary entry, see below for more information).

To set the version see the table showing the corresponding enum with version number.

| enum | PDF Version |
| --- | --- |
| PDFVersion::V1_7 | 1.7 |
| PDFVersion::V2_0 | 2.0 |

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
use pdf_composer::{PDFComposer, PDFDocInfoEntry, PDFVersion};

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