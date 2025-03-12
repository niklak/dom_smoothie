window.BENCHMARK_DATA = {
  "lastUpdate": 1741783106521,
  "repoUrl": "https://github.com/niklak/dom_smoothie",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "4706a288e481d4fea7a441a841407d0055ae139e",
          "message": ".github/workflows/benchmark.yml: update",
          "timestamp": "2025-02-17T12:38:00+02:00",
          "tree_id": "344ec706106025efb7a67d80a6f931718e3cb2fd",
          "url": "https://github.com/niklak/dom_smoothie/commit/4706a288e481d4fea7a441a841407d0055ae139e"
        },
        "date": 1739788786003,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3183409,
            "range": "± 44366",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12860163,
            "range": "± 332412",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 59372224,
            "range": "± 913762",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "96c7b66061a8bc9bc37743c47837f8972523b553",
          "message": "benches/parse.rs: update",
          "timestamp": "2025-02-17T13:03:16+02:00",
          "tree_id": "c8a1b7e9fc68fecfcecd8135a77af291895cacdd",
          "url": "https://github.com/niklak/dom_smoothie/commit/96c7b66061a8bc9bc37743c47837f8972523b553"
        },
        "date": 1739790287262,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3176245,
            "range": "± 53859",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12391450,
            "range": "± 337005",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 56131929,
            "range": "± 596050",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c95971401d91a702fb6fc5e24813024b9bd9937a",
          "message": "Merge pull request #64 from niklak/feature/refactor\n\n- Update `dom_query`'s version to `0.15.0`.\n- Minor code changes.",
          "timestamp": "2025-03-02T11:52:31+02:00",
          "tree_id": "49b81fb3689a3f191c9aea3144d8e7dc556d5230",
          "url": "https://github.com/niklak/dom_smoothie/commit/c95971401d91a702fb6fc5e24813024b9bd9937a"
        },
        "date": 1740909239942,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3195002,
            "range": "± 27596",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12427263,
            "range": "± 835095",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 55653100,
            "range": "± 570816",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a2d9207901f139a027fcf376c54adaf2ee99a0a7",
          "message": "Merge pull request #65 from niklak/feature/markdown-support\n\n- `Readability::parse` can now output text as `Markdown` in `Article::text_content` when `Config::text_mode` is set to `TextMode::Markdown`.",
          "timestamp": "2025-03-02T18:17:46+02:00",
          "tree_id": "780b28dddb0d630c2635878b920cfedd070fbae1",
          "url": "https://github.com/niklak/dom_smoothie/commit/a2d9207901f139a027fcf376c54adaf2ee99a0a7"
        },
        "date": 1740932335966,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3242587,
            "range": "± 80162",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12854079,
            "range": "± 284054",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 57723496,
            "range": "± 1437540",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2776f0f27dfe77715237bcc0f33d5680399e1098",
          "message": "Merge pull request #66 from niklak/feature/refactor-2\n\nMinor adjustment",
          "timestamp": "2025-03-03T12:28:58+02:00",
          "tree_id": "531bffb38694eb2f07477d3dc356725ccf1a707a",
          "url": "https://github.com/niklak/dom_smoothie/commit/2776f0f27dfe77715237bcc0f33d5680399e1098"
        },
        "date": 1740997790973,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3211050,
            "range": "± 41255",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12360253,
            "range": "± 201384",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 56606039,
            "range": "± 2609654",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "cdac7f211c06997233a042b5d695ff3c1ca63550",
          "message": "Cargo.toml: update",
          "timestamp": "2025-03-03T12:36:55+02:00",
          "tree_id": "5b77fb769e7c3b8004ca9f596f48344c192950b5",
          "url": "https://github.com/niklak/dom_smoothie/commit/cdac7f211c06997233a042b5d695ff3c1ca63550"
        },
        "date": 1740998306337,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3233773,
            "range": "± 13916",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12995875,
            "range": "± 423964",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 56952473,
            "range": "± 622049",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "105551b76f50dbbaf013a185969f5f3dc6bbc149",
          "message": "Merge pull request #67 from niklak/feature/empty-links\n\n- Link elements (`<a>`) without an `href` attribute and without child nodes are now removed from the article content during post-processing.\n- Changed how phrasing content determines wrapping some `<div>` element children with a `<p>` element. Now the element must contain some nodes to be wrapped.",
          "timestamp": "2025-03-06T13:52:39+02:00",
          "tree_id": "278d2a1507399c944867bf6621b885d74ba4ab82",
          "url": "https://github.com/niklak/dom_smoothie/commit/105551b76f50dbbaf013a185969f5f3dc6bbc149"
        },
        "date": 1741262013182,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3187636,
            "range": "± 43909",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12766966,
            "range": "± 405664",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 58549666,
            "range": "± 1741251",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4c3c8d29a602638e0e25ff31c263b1c8da5353d6",
          "message": "Merge pull request #68 from niklak/feature/refactor-3\n\n- Adapted the code to changes in the `dom_query`dependency.",
          "timestamp": "2025-03-09T14:26:09+02:00",
          "tree_id": "46ab9514fc8148f4449cf2440cbaa1f3a5850082",
          "url": "https://github.com/niklak/dom_smoothie/commit/4c3c8d29a602638e0e25ff31c263b1c8da5353d6"
        },
        "date": 1741523263976,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3064590,
            "range": "± 76761",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11992452,
            "range": "± 284573",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52714546,
            "range": "± 593707",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "fca2e33f4649f91540d51cc005f95470bae8de23",
          "message": "CHANGELOG.md: update version",
          "timestamp": "2025-03-10T14:07:27+02:00",
          "tree_id": "c5628034e3b6af06e502b6d4a62dfb640c8ad643",
          "url": "https://github.com/niklak/dom_smoothie/commit/fca2e33f4649f91540d51cc005f95470bae8de23"
        },
        "date": 1741608514331,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3069098,
            "range": "± 123961",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11473810,
            "range": "± 57086",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51903046,
            "range": "± 532252",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "morgenpurple@gmail.com",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "86be8e70dfeffc1febfb5ef219ea952afce2da34",
          "message": "Cargo.toml: update",
          "timestamp": "2025-03-10T14:08:11+02:00",
          "tree_id": "302423c7ec4d5c61b817c4f2f7b0da4a0033aa33",
          "url": "https://github.com/niklak/dom_smoothie/commit/86be8e70dfeffc1febfb5ef219ea952afce2da34"
        },
        "date": 1741608567070,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3072481,
            "range": "± 153233",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 12189583,
            "range": "± 212262",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 54745255,
            "range": "± 1218675",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bd95294612ab6f9ab2952671be65d20bbf0af4ce",
          "message": "Merge pull request #69 from niklak/dependabot/cargo/clap-4.5.32\n\nbuild(deps): bump clap from 4.5.31 to 4.5.32",
          "timestamp": "2025-03-12T14:37:13+02:00",
          "tree_id": "26236ac96e6d7e36e43e133012ad467aba3506ef",
          "url": "https://github.com/niklak/dom_smoothie/commit/bd95294612ab6f9ab2952671be65d20bbf0af4ce"
        },
        "date": 1741783106081,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3105941,
            "range": "± 125345",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11649269,
            "range": "± 44963",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52687004,
            "range": "± 1457773",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}