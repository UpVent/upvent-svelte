# UpVent Website Source

![GitHub issues](https://img.shields.io/github/issues/UpVent/upventrs)
![GitHub all releases](https://img.shields.io/github/downloads/UpVent/upventrs/total)
![GitHub](https://img.shields.io/github/license/UpVent/upventrs)
[![Node.js CI](https://github.com/UpVent/upventrs/actions/workflows/node.js.yml/badge.svg)](https://github.com/UpVent/upventrs/actions/workflows/node.js.yml)
[![Rust + Cargo](https://github.com/UpVent/upventrs/actions/workflows/rust.yml/badge.svg)](https://github.com/UpVent/upventrs/actions/workflows/rust.yml)
[![Mark stale issues and pull requests](https://github.com/UpVent/upventrs/actions/workflows/stale.yml/badge.svg)](https://github.com/UpVent/upventrs/actions/workflows/stale.yml)

This is the source code for the UpVent's website under upvent.codes.

Development occurs on the master branch and new branches are only opened when adding critical/unstable features. The main site is published by upvent making a fork of this project and adding some post-update commits in order to make deployment more secure.

## :warning: PLEASE CONSIDER THESE THINGS FIRST

1- Some content is in Spanish(MX), translate it at your own needs. The code itself is in English, only pertinent strings like html tag contents are in spanish.

2- There's dummy content in master branch, if you cannot afford to have experimental features in production, please use the release tags.

## Development Stack
- Rust :crab: (latest stable) + Rocket :rocket: (v0.5) powered backend.
- Svelte powered frontend.

## Philosophies
- **TRY TO KEEP IT SIMPLE**, we are already using two frameworks. As much as these technologies are useful, we don't want to contribute to the bloated web, keep things simple, secure and easy to maintain.
- Very tiny landing page, not full blown with features.
- Keep breaking changes to a minimum. Take more time if needed.

## Security

For any serious security related issues, please **do not open a public GitHub issue**. 

Instead, email them to [our issues email](mailto:upventmx@gmail.com). We provide a quick response of security reports within 24 hours and should apply patches ASAP (also, feel free to contribute a fix for the issue).

## Contributing

If you wish to continue please look at our [ISSUES](https://github.com/UpVent/upvent.codes/issues) tab, open a new Issue to request a feature or report a bug.

If this work helped you somehow, please leave us a :star: it will mean so much to us :heart:
