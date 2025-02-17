window.BENCHMARK_DATA = {
  "lastUpdate": 1739790287643,
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
      }
    ]
  }
}