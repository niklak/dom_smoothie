window.BENCHMARK_DATA = {
  "lastUpdate": 1743523920932,
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
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "56a94872325d9af7630aff288d8e4323199414c0",
          "message": "build(deps): bump once_cell from 1.20.3 to 1.21.0\n\nBumps [once_cell](https://github.com/matklad/once_cell) from 1.20.3 to 1.21.0.\n- [Changelog](https://github.com/matklad/once_cell/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/matklad/once_cell/compare/v1.20.3...v1.21.0)\n\n---\nupdated-dependencies:\n- dependency-name: once_cell\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2025-03-12T14:40:00+02:00",
          "tree_id": "62bd6b1b82cb211b0ce6b7363634ef4150155105",
          "url": "https://github.com/niklak/dom_smoothie/commit/56a94872325d9af7630aff288d8e4323199414c0"
        },
        "date": 1741783278541,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3063229,
            "range": "± 98731",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11490463,
            "range": "± 36310",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52356841,
            "range": "± 1149478",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "9eed3f62d069b55a3086d13dc6db676fbf626e23",
          "message": "build(deps): bump serde from 1.0.218 to 1.0.219\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.218 to 1.0.219.\n- [Release notes](https://github.com/serde-rs/serde/releases)\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.218...v1.0.219)\n\n---\nupdated-dependencies:\n- dependency-name: serde\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2025-03-12T14:43:28+02:00",
          "tree_id": "16a4cf287b27c8c407d0bfc207c5202cf53c5187",
          "url": "https://github.com/niklak/dom_smoothie/commit/9eed3f62d069b55a3086d13dc6db676fbf626e23"
        },
        "date": 1741783487242,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3064760,
            "range": "± 75739",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11521033,
            "range": "± 180081",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52132082,
            "range": "± 1194789",
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
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "2eaf1744c03725257ccbb6dc279c9e58472ec445",
          "message": "CHANGELOG.md: update",
          "timestamp": "2025-03-13T12:56:49+02:00",
          "tree_id": "458cf69762fe032b7b3b212bce10f9fb61c9b34f",
          "url": "https://github.com/niklak/dom_smoothie/commit/2eaf1744c03725257ccbb6dc279c9e58472ec445"
        },
        "date": 1741863474962,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3071453,
            "range": "± 126788",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11858425,
            "range": "± 459368",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52150972,
            "range": "± 810140",
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
          "id": "58a4c121bbf3e9e0281d750dda24599f2540a9df",
          "message": "Merge pull request #73 from niklak/feature/skip-svg\n\n- Ignoring `svg` elements during pre-filtering and element collection for scoring, improving performance for documents with many `svg` elements.",
          "timestamp": "2025-03-13T21:44:04+02:00",
          "tree_id": "21582ec9ad67409246da25ef0129d4d588a29f80",
          "url": "https://github.com/niklak/dom_smoothie/commit/58a4c121bbf3e9e0281d750dda24599f2540a9df"
        },
        "date": 1741895097724,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3087693,
            "range": "± 33432",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11510146,
            "range": "± 133317",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 53600860,
            "range": "± 1608193",
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
          "id": "f7a574b886152179a533ecbeedc1da1714a7bdc2",
          "message": "Merge pull request #74 from niklak/feature/parse-with-policy\n\n- Added `Readability::parse_with_policy` method, which performs one attempt to extract relevant content from an HTML document with `ParsePolicy`. This method consumes **significantly** less memory than `Readability::parse` but it is also less precise, as `Readability::parse` is able to perform more than one attempt.\n- Added the `dom_smoothie_js::Readability::parse_with_policy` method, a wrapper around `dom_smoothie::Readability::parse_with_policy`.",
          "timestamp": "2025-03-15T15:23:51+02:00",
          "tree_id": "8ecb6ef8928f308a4c449fe17669a887392432a8",
          "url": "https://github.com/niklak/dom_smoothie/commit/f7a574b886152179a533ecbeedc1da1714a7bdc2"
        },
        "date": 1742045081689,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3113364,
            "range": "± 86081",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11884268,
            "range": "± 582838",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 53008646,
            "range": "± 363385",
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
          "id": "f6b21b14e42c9b17d04e77827da8baf0421fb7e9",
          "message": "Merge pull request #75 from niklak/feature/refactor-4\n\n- Revised `has_ancestor_tag`\n- Revised `should_clean_conditionally`\n- Adjusted `MATCHER_DIALOGS`\n- Minor changes",
          "timestamp": "2025-03-16T17:39:12+02:00",
          "tree_id": "dac0c4d44a4442f54c26ae29859328210b7204b0",
          "url": "https://github.com/niklak/dom_smoothie/commit/f6b21b14e42c9b17d04e77827da8baf0421fb7e9"
        },
        "date": 1742139608814,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3059810,
            "range": "± 14298",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11443722,
            "range": "± 423273",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51878776,
            "range": "± 627293",
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
          "id": "2bb67b410dbb8685afbf52dc75d5b4d2a32e2f14",
          "message": "tests/parse_policy.rs: update",
          "timestamp": "2025-03-17T09:58:54+02:00",
          "tree_id": "ae4391df35646f9d835231acfb77a3736c6f7a19",
          "url": "https://github.com/niklak/dom_smoothie/commit/2bb67b410dbb8685afbf52dc75d5b4d2a32e2f14"
        },
        "date": 1742198389858,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3063425,
            "range": "± 13451",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11450835,
            "range": "± 287709",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52283708,
            "range": "± 1898342",
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
          "id": "a4051a309a32ccaba035a3c1bb6381fedcb62e16",
          "message": "Merge pull request #76 from niklak/feature/mark-data-tables\n\nsrc/prep_article.rs: revise `mark_data_tables`",
          "timestamp": "2025-03-17T11:21:00+02:00",
          "tree_id": "6b03e4ee2672a68034e40baf0778d9aee1b20603",
          "url": "https://github.com/niklak/dom_smoothie/commit/a4051a309a32ccaba035a3c1bb6381fedcb62e16"
        },
        "date": 1742203316181,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3065449,
            "range": "± 56448",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11434398,
            "range": "± 291440",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52037427,
            "range": "± 535654",
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
          "id": "98076ba8714f0fa9f75876bea918b50b4d8e20a2",
          "message": "Cargo.toml: update version",
          "timestamp": "2025-03-17T11:31:15+02:00",
          "tree_id": "5663efe380c5dd6ede7ea59a7ed9c6e594f2d2a6",
          "url": "https://github.com/niklak/dom_smoothie/commit/98076ba8714f0fa9f75876bea918b50b4d8e20a2"
        },
        "date": 1742203961842,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 3046918,
            "range": "± 28674",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11352684,
            "range": "± 69352",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 54682401,
            "range": "± 1539597",
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
          "id": "c77f6f4d951681cde23283f7376fe3adec50bde6",
          "message": "Merge pull request #77 from niklak/feature/refactor-5\n\nInternal code changes",
          "timestamp": "2025-03-21T16:21:41+02:00",
          "tree_id": "0b0a56fcd4d16ad7754e19d03fe0751307a38f5c",
          "url": "https://github.com/niklak/dom_smoothie/commit/c77f6f4d951681cde23283f7376fe3adec50bde6"
        },
        "date": 1742567004287,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2855075,
            "range": "± 42273",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11123794,
            "range": "± 273466",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52409578,
            "range": "± 905572",
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
          "id": "91ac9c1b4ef2be7ae1347b81b452ac86f24a2142",
          "message": "Merge pull request #78 from niklak/feature/refactor-6\n\n- Updated the internal function `fix_lazy_images` to improve how it checks whether an element's class attribute contains the word `lazy`.",
          "timestamp": "2025-03-24T23:11:52+02:00",
          "tree_id": "254f61621af6b100fdf9589c7f8a056823f870d0",
          "url": "https://github.com/niklak/dom_smoothie/commit/91ac9c1b4ef2be7ae1347b81b452ac86f24a2142"
        },
        "date": 1742850770900,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2848193,
            "range": "± 17090",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11097349,
            "range": "± 340380",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51332900,
            "range": "± 514549",
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
          "id": "ae62e4b0b8fdb07c1a0561c8ea0a3c5f14681638",
          "message": "CHANGELOG.md: update",
          "timestamp": "2025-03-27T13:07:12+02:00",
          "tree_id": "7685e425d6075fda5f3ac22bdc0848df4e756609",
          "url": "https://github.com/niklak/dom_smoothie/commit/ae62e4b0b8fdb07c1a0561c8ea0a3c5f14681638"
        },
        "date": 1743073708994,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2857018,
            "range": "± 80131",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11166360,
            "range": "± 319968",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51521911,
            "range": "± 314339",
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
          "id": "4a89db687b8e2918c5acb9899d9dc04d03f86256",
          "message": "Merge pull request #79 from niklak/feature/min-score-adjust\n\n- Added the `Config::min_score_to_adjust` option, which allows controlling the minimum score required for adjustment during the scoring process. Only nodes with a score higher than this value will be adjusted by their link density. Thus, the higher the value, the faster the scoring process.",
          "timestamp": "2025-03-28T12:16:44+02:00",
          "tree_id": "427a1bb9087b1c8273a7b508910d6686adae120a",
          "url": "https://github.com/niklak/dom_smoothie/commit/4a89db687b8e2918c5acb9899d9dc04d03f86256"
        },
        "date": 1743157105598,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2860551,
            "range": "± 17471",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11086272,
            "range": "± 256829",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51988377,
            "range": "± 1430109",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2868622,
            "range": "± 17786",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9775631,
            "range": "± 437769",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 54364680,
            "range": "± 2018945",
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
          "id": "d52f56a1bccad47a21cc313ac0d68d7566c577ae",
          "message": "`CLASSES_NEGATIVE` and `CLASSES_POSITIVE` are `&[&str]` now (#80)\n\n* src/glob.rs: `CLASSES_NEGATIVE` and `CLASSES_POSITIVE` are `&[&str]` now\n* src/glob.rs: `CLASSES_NEGATIVE_WORDS` is `&[&str]` now",
          "timestamp": "2025-03-29T12:53:09+02:00",
          "tree_id": "b32fd44e9bf70dfa1d96cca9d9a400944dcc495e",
          "url": "https://github.com/niklak/dom_smoothie/commit/d52f56a1bccad47a21cc313ac0d68d7566c577ae"
        },
        "date": 1743245704856,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2840687,
            "range": "± 36686",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11117970,
            "range": "± 260590",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51153423,
            "range": "± 915607",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2862841,
            "range": "± 28057",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9667961,
            "range": "± 182039",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 51185118,
            "range": "± 1298874",
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
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "092456a9a9bfd8737d0e94d1d45ba36248f08ae4",
          "message": "dom-smoothie-js: switch to `lol_alloc`",
          "timestamp": "2025-03-31T16:13:37+03:00",
          "tree_id": "59ea260d4cf79aa9c465c5fcb78fa2683df977ff",
          "url": "https://github.com/niklak/dom_smoothie/commit/092456a9a9bfd8737d0e94d1d45ba36248f08ae4"
        },
        "date": 1743426925529,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2842785,
            "range": "± 85287",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11189782,
            "range": "± 304315",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 52479531,
            "range": "± 1318410",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2845284,
            "range": "± 22240",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9718787,
            "range": "± 52886",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 52457271,
            "range": "± 1983001",
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
          "id": "65c1fa23dd39d61f40e636c1e95128e8e9c8866a",
          "message": "Merge pull request #83 from niklak/feature/aho-corasick\n\n- Implemented the `aho-corasick` feature, enabling the use of the `aho-corasick` crate for defining unlikely candidates and for the node scoring process. This can speed up the overall parsing process by 5-10% in some cases, at the cost of slightly higher memory usage and an increase in binary size.",
          "timestamp": "2025-04-01T15:23:34+03:00",
          "tree_id": "a8274394f9ccae31d994b7916cc0d6f32cc879f4",
          "url": "https://github.com/niklak/dom_smoothie/commit/65c1fa23dd39d61f40e636c1e95128e8e9c8866a"
        },
        "date": 1743510316923,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2849433,
            "range": "± 19421",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11120643,
            "range": "± 67625",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 55120115,
            "range": "± 1086066",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2871437,
            "range": "± 39658",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9821078,
            "range": "± 79784",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 55001541,
            "range": "± 914260",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "gnk667@proton.me",
            "name": "Mykola Humanov",
            "username": "niklak"
          },
          "distinct": true,
          "id": "434aa25b82c41c82846183a5998dfbb96f7f3e35",
          "message": "build(deps): bump dom_query from 0.16.0 to 0.17.0\n\nBumps [dom_query](https://github.com/niklak/dom_query) from 0.16.0 to 0.17.0.\n- [Release notes](https://github.com/niklak/dom_query/releases)\n- [Changelog](https://github.com/niklak/dom_query/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/niklak/dom_query/compare/0.16.0...0.17.0)\n\n---\nupdated-dependencies:\n- dependency-name: dom_query\n  dependency-version: 0.17.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2025-04-01T15:26:53+03:00",
          "tree_id": "4da32dc398175ce8d4b8686e1e9c7edcbc62ea54",
          "url": "https://github.com/niklak/dom_smoothie/commit/434aa25b82c41c82846183a5998dfbb96f7f3e35"
        },
        "date": 1743510515816,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2816328,
            "range": "± 78793",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11039067,
            "range": "± 129544",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51560264,
            "range": "± 1138262",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2824570,
            "range": "± 18481",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9605437,
            "range": "± 62801",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 51207577,
            "range": "± 247746",
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
          "id": "f2d12dcf1b7bfaa189e523becbc9ed9620881a0c",
          "message": "chore(release): bump version to 0.10.0 and update CHANGELOG",
          "timestamp": "2025-04-01T19:10:22+03:00",
          "tree_id": "7b48108af641ba1e598c3d603a19078c2dec6859",
          "url": "https://github.com/niklak/dom_smoothie/commit/f2d12dcf1b7bfaa189e523becbc9ed9620881a0c"
        },
        "date": 1743523919962,
        "tool": "cargo",
        "benches": [
          {
            "name": "dom_smoothie/parse/small",
            "value": 2839679,
            "range": "± 145796",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium",
            "value": 11215238,
            "range": "± 181451",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large",
            "value": 51463009,
            "range": "± 1655584",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/small, min score to adjust 10",
            "value": 2850692,
            "range": "± 59429",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/medium, min score to adjust 10",
            "value": 9871899,
            "range": "± 95350",
            "unit": "ns/iter"
          },
          {
            "name": "dom_smoothie/parse/large, min score to adjust 10",
            "value": 51443750,
            "range": "± 1232117",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}