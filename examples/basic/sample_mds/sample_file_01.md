---

# Front Matter (YAML)

author: "Bob Wibble"
keywords: "PDF rust crates"
language: "en-GB"
title: "Sample file 01"
description: "A sample page."
random: "The quick brown fox jumps over the lazy dog."

---

# My first markdown

The quick brown fox jumps over the lazy dog.

---

Example of replace yaml value (title): {{title}} ---

Example of replace yaml value (title) for a second time: {{title}} ---

Let us bring other YAML values into play:
* Author = {{author}}
* Language = {{lang uage}} - this will not be substituted
* Random = {{random}}

***

Example of yaml value not found (bob): {{bob}}
