import { Readability, ParsePolicy } from "dom-smoothie-js";
import { readFileSync } from "node:fs";

function main() {
  const content = readFileSync("/home/niklak3/lab/rust/pathfinders/new/dom_smoothie/test-pages/rustwiki_2024.html", "utf8");
  const document_url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

  console.time("Readability.parse");
  const res = new Readability(content, document_url, null).parse();
  console.timeEnd("Readability.parse");
  console.log(res.title);
}

main();