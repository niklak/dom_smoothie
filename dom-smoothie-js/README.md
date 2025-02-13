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




## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).