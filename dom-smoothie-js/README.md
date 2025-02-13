# DOM-SMOOTHIE-JS
> `dom-smoothie-js` is a nodejs package for extracting readable content from web pages. 
> It is a wrapper around the rust [dom_smoothie](https://github.com/niklak/dom_smoothie) crate.


## Configuration
| Parameter                  | Type                       | Default Value                      | Description |
|-----------------------------|---------------------------|------------------------------------|-------------|
| keep_classes               | `boolean`                 | `false`                            | Keep all classes in the document |
| classes_to_preserve        | `Array<string>`           | `[]`                               | List of classes to preserve |
| max_elements_to_parse      | `number`                  | `0`                                | Maximum number of elements to parse |
| disable_json_ld            | `boolean`                 | `false`                            | Disable JSON-LD extraction |
| n_top_candidates           | `number`                  | `5`                                | Number of top candidates to consider |
| char_threshold             | `number`                  | `500`                              | Character threshold for content extraction |
| readable_min_score         | `number` (float)          | `20.0`                             | Minimum score required for readability check |
| readable_min_content_length| `number`                  | `140`                              | Minimum content length for readability check |
| candidate_select_mode      | `'Readability' \| 'DomSmoothie'` | `'Readability'`                 | Candidate selection mode |
| text_mode                  | `'Raw' \| 'Formatted'`    | `'Raw'`                            | Text output mode, either raw or formatted |

### Example Object with Default Parameters

```javascript
const config = {
  keep_classes: false,
  classes_to_preserve: [],
  max_elements_to_parse: 0,
  disable_json_ld: false,
  n_top_candidates: 5,
  char_threshold: 500,
  readable_min_score: 20.0,
  readable_min_content_length: 140,
  candidate_select_mode: 'Readability',
  text_mode: 'Raw'
};
```

## Examples


<details>
    <summary><b>Readability.parse — a basic example</b></summary>


```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");
  const document_url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";
  const cfg = {
    classes_to_preserve: ["caption"],
  }

  // document_url and cfg
  const article = new Readability(content, document_url, cfg).parse();
  console.log("Title:", article.title);
  console.log("Byline:", article.byline);
  console.log("Length:", article.length);
  console.log("Excerpt:", article.excerpt);
  console.log("Site Name:", article.site_name);
  console.log("Dir:", article.dir);
  console.log("Published Time:", article.published_time);
  console.log("Modified Time:", article.modified_time);
  console.log("Image:", article.image);
  // This uri can be taken only from ld+json
  console.log("URL:", article.url);

  // Skipping article.content since it is too large.
  //console.log("HTML Content:", article.content);

  // Skipping article.text_content since it is too large.
  //console.log("Text Content:", article.text_content);
}

main();
```
</details>


<details>
    <summary><b>Parsing only article`s title</b></summary>


```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");


  // You can parse only the metadata without parsing the article content.
  const readability = new Readability(content, null, null);

  // Parse only the title without extracting the full content.
  const title = readability.get_article_title();
  console.log("Title:", title);

  // However, this title may differ from `metadata.title`,
  // as `metadata.title` first attempts to extract the title from the metadata
  // and falls back to `Readability::get_article_title` if unavailable.

}

main();
```
</details>


<details>
    <summary><b>Parsing only metadata</b></summary>


```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");

  const cfg = {
    disable_json_ld: false,
  };

  // You can parse only metadata without parsing the article content
  const readability = new Readability(content, null, cfg);

  // <script type="application/ld+json"> may contain some useful information,
  // but usually it is not enough.
  const ld_meta = readability.parse_json_ld();

  console.log("LD META:", ld_meta);

  // Under the hood, `Readability::parse` passes the metadata obtained from `Readability::parse_json_ld`
  // as the basis to `Readability::get_article_metadata`. But this is not necessary.
  const meta = readability.get_article_metadata(ld_meta);

  console.log("META:", meta);

  // Some fields of Metadata may be missing because they can be assigned
  // during the Readability::parse process.
  // This applies to `excerpt`, `byline`, and `dir`.
}

main();
```
</details>


<details>
    <summary><b>Checking if content is readable</b></summary>


```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");

  // you can specify optional parameters for `Readability.is_probably_readable`.
  const cfg = {
    readable_min_score: 20.0,
    readable_min_content_length: 140,
  };

  const readability = new Readability(content, null, cfg);

  // There is a way to perform a quick check to determine
  // if the document is readable before cleaning and parsing it.
  // After calling `Readability::parse`, it may show different results,
  // but calling it after parsing would be nonsensical.
  if (readability.is_probably_readable()) {
    let article = readability.parse();
    console.log("Title:", article.title);
    console.log("Byline:", article.byline);
    console.log("Site Name:", article.site_name);
    console.log("URL:", article.url);
    // and so on...
  }
}

main();
```
</details>


<details>
    <summary><b>Using an alternative approach to selecting the best candidate</b></summary>


```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");

  const cfg = {
    candidate_select_mode: "DomSmoothie",
  };

  const readability = new Readability(content, null, cfg);

  const article = readability.parse();
  console.log("Text Content:", article.text_content);
}

main();
```
</details>



<details>
    <summary><b>Formatted text content</b></summary>

By default, the text content is output as-is, without formatting, 
preserving whitespace from the original HTML document. 
Depending on the document's initial markup, this can be quite verbose and inconvenient.

But it is also possible to retrieve formatted text content. 
To enable this, set `text_mode: TextMode::Formatted` in the config.
This formatting is simple; for example, it does not account for table formatting.
It is certainly nowhere near markdown-level, but the result is noticeably 
cleaner than without formatting.

```javascript
import { Readability } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("test_data/rustwiki_2024.html", "utf8");

  const cfg = {
    text_mode: "Formatted",
  };

  const readability = new Readability(content, null, cfg);

  const article = readability.parse();
  console.log("Text Content:", article.text_content);
}

main();
```
</details>

## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).