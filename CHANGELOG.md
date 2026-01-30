# Changelog

## [6.0.2](https://github.com/cailloumajor/frontend-utils-wasm/compare/v6.0.1...v6.0.2) (2026-01-30)


### Bug Fixes

* **deps:** update rust docker tag to v1.93.0 ([afd1066](https://github.com/cailloumajor/frontend-utils-wasm/commit/afd1066c3c00d4f663d86dccd5cdf582ada9e51b))
* **deps:** update wasm-bindgen, wasmbuild and deno deps ([e74b351](https://github.com/cailloumajor/frontend-utils-wasm/commit/e74b35182a190eb86b225eff49beb516d63bc032))

## [6.0.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v6.0.0...v6.0.1) (2026-01-01)


### Bug Fixes

* **deps:** specify full dependencies versions ([27233a9](https://github.com/cailloumajor/frontend-utils-wasm/commit/27233a9223630aeecc05cbe73983c4f86566428e))
* **deps:** update rust crate csscolorparser to 0.8 ([f452c9a](https://github.com/cailloumajor/frontend-utils-wasm/commit/f452c9aaa71f7c4db65eb48cff5d2aff019bc173))
* **deps:** update rust docker tag to v1.90.0 ([779a336](https://github.com/cailloumajor/frontend-utils-wasm/commit/779a3368bc3b95d90e348d45f62ab01651e5e04c))
* **deps:** update rust docker tag to v1.91.0 ([2d785b1](https://github.com/cailloumajor/frontend-utils-wasm/commit/2d785b1a68c0b76ec9be75ef56f792f044c2496e))
* **deps:** update rust docker tag to v1.91.1 ([0dd0b27](https://github.com/cailloumajor/frontend-utils-wasm/commit/0dd0b2759fee7a78f733e02db9c468590742d0df))
* **deps:** update rust docker tag to v1.92.0 ([f5be95e](https://github.com/cailloumajor/frontend-utils-wasm/commit/f5be95e45819fd008889ef1f424226821ccd931e))
* **deps:** upgrade wasm-bindgen and wasmbuild ([0035e91](https://github.com/cailloumajor/frontend-utils-wasm/commit/0035e914bfb53ce6eb79b0049b35cb63ae3373cf))

## [6.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v5.1.1...v6.0.0) (2025-09-09)


### ⚠ BREAKING CHANGES

* separate setting the data to draw and drawing
* remove `drawed` event emission
* make the draw method sync

### Features

* separate setting the data to draw and drawing ([dd99290](https://github.com/cailloumajor/frontend-utils-wasm/commit/dd99290ddb4051ab74eb5a81f9e6d2cdf0c694f1))


### Bug Fixes

* make the draw method sync ([7c77b74](https://github.com/cailloumajor/frontend-utils-wasm/commit/7c77b7411c1983839ced574e20f44ece8b9a0630))
* remove `drawed` event emission ([6502060](https://github.com/cailloumajor/frontend-utils-wasm/commit/6502060cd747f838c0a6172d6d83982a8d683bc5))

## [5.1.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v5.1.0...v5.1.1) (2025-08-14)


### Bug Fixes

* **deps:** update rust docker tag to v1.89.0 ([#140](https://github.com/cailloumajor/frontend-utils-wasm/issues/140)) ([71b2cac](https://github.com/cailloumajor/frontend-utils-wasm/commit/71b2caca71922583692816b31536859a3aa25ae7))

## [5.1.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v5.0.2...v5.1.0) (2025-07-07)


### Features

* allow CSS color strings for palette and canvas element ([22a079e](https://github.com/cailloumajor/frontend-utils-wasm/commit/22a079e57c982a6a6e2a3e9d58c925a27c06ae8e))


### Bug Fixes

* **deps:** update rust crate colorsys to 0.7 ([#127](https://github.com/cailloumajor/frontend-utils-wasm/issues/127)) ([4023126](https://github.com/cailloumajor/frontend-utils-wasm/commit/4023126ab6d4c1058fae89bd6e2d04726e008ca5))
* **deps:** update rust docker tag to v1.88.0 ([#130](https://github.com/cailloumajor/frontend-utils-wasm/issues/130)) ([9920366](https://github.com/cailloumajor/frontend-utils-wasm/commit/9920366084a625739bb5077c6819be546ff23e60))

## [5.0.2](https://github.com/cailloumajor/frontend-utils-wasm/compare/v5.0.1...v5.0.2) (2025-05-30)


### Miscellaneous Chores

* release 5.0.2 ([aee6755](https://github.com/cailloumajor/frontend-utils-wasm/commit/aee675590faaab14775df2ac604bd0f5afde7250))

## [5.0.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v5.0.0...v5.0.1) (2025-05-30)


### Miscellaneous Chores

* release 5.0.1 ([79cbcb1](https://github.com/cailloumajor/frontend-utils-wasm/commit/79cbcb1f1f0a155550f13195adebcb1a815b36d5))

## [5.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v4.0.0...v5.0.0) (2025-05-23)


### ⚠ BREAKING CHANGES

* change the name of the configuration object and add doc comments ([#120](https://github.com/cailloumajor/frontend-utils-wasm/issues/120))

### Bug Fixes

* change the name of the configuration object and add doc comments ([#120](https://github.com/cailloumajor/frontend-utils-wasm/issues/120)) ([3da512a](https://github.com/cailloumajor/frontend-utils-wasm/commit/3da512a21e9139e62dfbc9015be0f205fb89e1bb))

## [4.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/frontend-utils-wasm-v3.0.3...frontend-utils-wasm-v4.0.0) (2025-05-20)


### ⚠ BREAKING CHANGES

* use deno and wasmbuild ([#116](https://github.com/cailloumajor/frontend-utils-wasm/issues/116))

### Features

* use deno and wasmbuild ([#116](https://github.com/cailloumajor/frontend-utils-wasm/issues/116)) ([bd91075](https://github.com/cailloumajor/frontend-utils-wasm/commit/bd9107587d1e5aaa8cf74801c4c3964079a6ac3c))


### Bug Fixes

* **deps:** update rust docker tag to v1.85.1 ([#112](https://github.com/cailloumajor/frontend-utils-wasm/issues/112)) ([6d60918](https://github.com/cailloumajor/frontend-utils-wasm/commit/6d60918918ff0fcd2eb98943a8b0adc2af5bc893))
* **deps:** update rust docker tag to v1.86.0 ([#114](https://github.com/cailloumajor/frontend-utils-wasm/issues/114)) ([566e9c5](https://github.com/cailloumajor/frontend-utils-wasm/commit/566e9c5a161c6ea060dbc8500f4bbf6d76cb46ae))
* **deps:** update rust docker tag to v1.87.0 ([#117](https://github.com/cailloumajor/frontend-utils-wasm/issues/117)) ([1fcd2e2](https://github.com/cailloumajor/frontend-utils-wasm/commit/1fcd2e2a8fb4d32e5b34b5eeb36aaa604319e603))

## [3.0.3](https://github.com/cailloumajor/frontend-utils-wasm/compare/v3.0.2...v3.0.3) (2025-03-03)


### Bug Fixes

* **deps:** update rust docker tag to v1.84.1 ([a81295b](https://github.com/cailloumajor/frontend-utils-wasm/commit/a81295b2a15784bcc269b0394c573658da3442a1))
* **deps:** update rust docker tag to v1.84.1 ([d86a73b](https://github.com/cailloumajor/frontend-utils-wasm/commit/d86a73bb3ef1d5f576731bb3722046009d85824c))
* **deps:** upgrade Rust edition, version and dependencies ([eb53696](https://github.com/cailloumajor/frontend-utils-wasm/commit/eb53696c13a7c30d2016ea89fdae48def4288f8c))
* **deps:** upgrade Rust edition, version and dependencies ([b1a56d1](https://github.com/cailloumajor/frontend-utils-wasm/commit/b1a56d18232db1bb2ccacdbc6577eedba1b428ca))

## [3.0.2](https://github.com/cailloumajor/frontend-utils-wasm/compare/v3.0.1...v3.0.2) (2024-08-12)


### Bug Fixes

* **deps:** update Rust docker tag to bookworm ([50758fa](https://github.com/cailloumajor/frontend-utils-wasm/commit/50758fa4ba6e234e2b94894a9ed5cdad67d2aabe))
* **deps:** update rust docker tag to v1.80.1 ([1fd206a](https://github.com/cailloumajor/frontend-utils-wasm/commit/1fd206ac1c307cbb5ab5ae31895c228be70ab099))
* **deps:** upgrade Rust to 1.77.2 ([2685668](https://github.com/cailloumajor/frontend-utils-wasm/commit/2685668ce7a2aaa4ab889dfa00e9aeadab290646))

## [3.0.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v3.0.0...v3.0.1) (2023-08-18)


### Bug Fixes

* **deps:** update rust crate rmp-serde to 1.1.2 ([6db89ca](https://github.com/cailloumajor/frontend-utils-wasm/commit/6db89ca4b763166d9194314f75332e983660cc3f))
* **deps:** update rust crate serde to 1.0.183 ([2a9281b](https://github.com/cailloumajor/frontend-utils-wasm/commit/2a9281b369713322fa93c77eb394f26d042ac368))
* **deps:** update rust crate thiserror to 1.0.47 ([3bc6ab1](https://github.com/cailloumajor/frontend-utils-wasm/commit/3bc6ab13c8c3e663d5964dfe3d58a9c176037af0))
* **deps:** update rust docker tag to v1.71.1 ([c8bc496](https://github.com/cailloumajor/frontend-utils-wasm/commit/c8bc49674cdc3ab5ee352cf2b39ed26b54970f4b))

## [3.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.5...v3.0.0) (2023-07-09)


### ⚠ BREAKING CHANGES

* rename the project
* just draw timeline from MessagePack data

### Features

* just draw timeline from MessagePack data ([61247af](https://github.com/cailloumajor/frontend-utils-wasm/commit/61247afce31ca1dd1b08439198eb140007188b51))


### Bug Fixes

* **deps:** update rust crate chrono to 0.4.26 ([a0ebd50](https://github.com/cailloumajor/frontend-utils-wasm/commit/a0ebd502820150777450193366b597c728f1537b))
* **deps:** update rust crate csv to 1.2.2 ([b709445](https://github.com/cailloumajor/frontend-utils-wasm/commit/b7094454810176a60147f99d1dcffd6770e99fe2))
* **deps:** update rust crate itertools to 0.11.0 ([802b697](https://github.com/cailloumajor/frontend-utils-wasm/commit/802b697b2a97c86d4f59dda3f62941354210b5e4))
* **deps:** update rust crate plotters to 0.3.5 ([e08b775](https://github.com/cailloumajor/frontend-utils-wasm/commit/e08b7758161310fb9b4060b5234a3957a7b019bb))
* **deps:** update rust crate serde to 1.0.164 ([7d3e912](https://github.com/cailloumajor/frontend-utils-wasm/commit/7d3e912277e2d722cb9da0dc9656d3a2cc4ee07b))
* **deps:** update rust docker tag to v1.70.0 ([6f4d55b](https://github.com/cailloumajor/frontend-utils-wasm/commit/6f4d55bf9b7158e384a4a5f15cf8778c304b0f63))
* **deps:** update rust-wasm-bindgen monorepo ([ead24ef](https://github.com/cailloumajor/frontend-utils-wasm/commit/ead24ef59b3328c1714f0387480971eea3a2ff88))
* prevent flux query string copying ([557c5f9](https://github.com/cailloumajor/frontend-utils-wasm/commit/557c5f9fd9096656d93f143aba7863250b935bca))
* use wasm_bindgen::UnwrapThrowExt ([3b83550](https://github.com/cailloumajor/frontend-utils-wasm/commit/3b8355035762e154e5ecf66c45c4148d75374048))


### Miscellaneous Chores

* rename the project ([280ac5e](https://github.com/cailloumajor/frontend-utils-wasm/commit/280ac5e052c8e1e4ec53a9d8a04539794ab20f87))

## [2.2.5](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.4...v2.2.5) (2023-05-26)


### Bug Fixes

* **deps:** update rust crate tsify to 0.4.5 ([89b7eed](https://github.com/cailloumajor/frontend-utils-wasm/commit/89b7eedbc1747591b3da94a29ba3e118533f092b))

## [2.2.4](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.3...v2.2.4) (2023-05-17)


### Bug Fixes

* **deps:** update rust crate serde to 1.0.160 ([2a4a9bc](https://github.com/cailloumajor/frontend-utils-wasm/commit/2a4a9bc9bf8b77937b282b65ba94b2d2f997b69f))
* **deps:** update rust crate serde to 1.0.162 ([2df7c58](https://github.com/cailloumajor/frontend-utils-wasm/commit/2df7c58f4ea4cebaefe1a3751be0ce65e6b68e91))
* **deps:** update rust crate serde to 1.0.163 ([2243909](https://github.com/cailloumajor/frontend-utils-wasm/commit/2243909d99f04c209bd40219429739e5f40da4d4))
* **deps:** update rust docker tag to v1.69.0 ([7f1f26b](https://github.com/cailloumajor/frontend-utils-wasm/commit/7f1f26b6438370065f3685ff2c19ec4ee36a9595))
* **deps:** update rust-wasm-bindgen monorepo ([8747a2f](https://github.com/cailloumajor/frontend-utils-wasm/commit/8747a2fed615d7f061907395e4b120324cd2c671))

## [2.2.3](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.2...v2.2.3) (2023-03-31)


### Bug Fixes

* do not panic on canvas color parsing error ([a575ee7](https://github.com/cailloumajor/frontend-utils-wasm/commit/a575ee7a9db4244bdcb89899e546244c31ab9840))

## [2.2.2](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.1...v2.2.2) (2023-03-29)


### Bug Fixes

* **deps:** update rust crate chrono to 0.4.24 ([4df2e18](https://github.com/cailloumajor/frontend-utils-wasm/commit/4df2e1821c787dffdffe8ac4c9043e93c800c5dd))
* **deps:** update rust crate csv to 1.2.1 ([ff2cfb4](https://github.com/cailloumajor/frontend-utils-wasm/commit/ff2cfb4dbffa2dfc58458c5b440ee30b50fd6eb5))
* **deps:** update rust crate serde to 1.0.153 ([d35a66c](https://github.com/cailloumajor/frontend-utils-wasm/commit/d35a66c2554bb271f122e2ec07cfa5c8e4cd9e79))
* **deps:** update rust crate serde to 1.0.154 ([f5da97c](https://github.com/cailloumajor/frontend-utils-wasm/commit/f5da97cc364ef697138e66de01c553e62f12fb35))
* **deps:** update rust crate serde to 1.0.155 ([2c04391](https://github.com/cailloumajor/frontend-utils-wasm/commit/2c04391a6044c67db7e8c348331d19fcca4364ac))
* **deps:** update rust crate serde to 1.0.156 ([f4fea90](https://github.com/cailloumajor/frontend-utils-wasm/commit/f4fea90aa71e90bd6bf37081a642964eed571fb3))
* **deps:** update rust crate serde to 1.0.157 ([de49c99](https://github.com/cailloumajor/frontend-utils-wasm/commit/de49c99d6bf0cabee426ced1450ed2f6428540a1))
* **deps:** update rust crate serde to 1.0.158 ([c6e431b](https://github.com/cailloumajor/frontend-utils-wasm/commit/c6e431b8ddff424f5ea9e559d29bdb0f1d7efcee))
* **deps:** update rust crate serde to 1.0.159 ([14f35e8](https://github.com/cailloumajor/frontend-utils-wasm/commit/14f35e8361519023768688868e16c9b183ec575d))
* **deps:** update rust crate thiserror to 1.0.39 ([8497662](https://github.com/cailloumajor/frontend-utils-wasm/commit/8497662506e23d4de538fc99e4592d51073df544))
* **deps:** update rust crate thiserror to 1.0.40 ([02a1bd0](https://github.com/cailloumajor/frontend-utils-wasm/commit/02a1bd0f206eb6ca1145aa88c9d75f488e5303a8))
* **deps:** update rust docker tag to v1.68.0 ([339b933](https://github.com/cailloumajor/frontend-utils-wasm/commit/339b9335776f7234f058a3ae3d0a21cb4111b3eb))
* **deps:** update rust docker tag to v1.68.1 ([83a62fb](https://github.com/cailloumajor/frontend-utils-wasm/commit/83a62fb34cd83e5a125c6a7adfdc2e759fc03ded))
* **deps:** update rust docker tag to v1.68.2 ([e2564a1](https://github.com/cailloumajor/frontend-utils-wasm/commit/e2564a1849cbd4b9d1c695b36c34640fd98d2fb3))

## [2.2.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.2.0...v2.2.1) (2023-02-27)


### Bug Fixes

* change labels emphasis characters ([98ebcd6](https://github.com/cailloumajor/frontend-utils-wasm/commit/98ebcd6ac4fe9f1b5743d4fda9292a5a477cb7d7))

## [2.2.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.1.1...v2.2.0) (2023-02-24)


### Features

* add emphasis on configured labels ([b80cd8d](https://github.com/cailloumajor/frontend-utils-wasm/commit/b80cd8d2a94d7b2efdd4cbcbc63da67a1e6f60e5))


### Bug Fixes

* **deps:** update rust crate csv to 1.2.0 ([13b5d16](https://github.com/cailloumajor/frontend-utils-wasm/commit/13b5d16b7c2892ca751280a56d26a18e7ea58e79))
* **deps:** update rust docker tag to v1.67.1 ([820ccf8](https://github.com/cailloumajor/frontend-utils-wasm/commit/820ccf8004af1a718c03758a2af543f9b7bf34d1))

## [2.1.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.1.0...v2.1.1) (2023-02-06)


### Bug Fixes

* **deps:** update rust-wasm-bindgen monorepo ([d784b20](https://github.com/cailloumajor/frontend-utils-wasm/commit/d784b20b0cfcaa13447fbb48bf0e7862118f64bf))

## [2.1.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v2.0.0...v2.1.0) (2023-02-01)


### Features

* add X axis configuration options ([2e69c75](https://github.com/cailloumajor/frontend-utils-wasm/commit/2e69c75973e628d39abde7bed5358345521fff3b))


### Bug Fixes

* **deps:** update rust crate colorsys to 0.6.7 ([fb86b2b](https://github.com/cailloumajor/frontend-utils-wasm/commit/fb86b2bc9de8e2c04439a36d0a32205a0ed259fb))
* **deps:** update rust crate gloo-net to 0.2.6 ([1ba0679](https://github.com/cailloumajor/frontend-utils-wasm/commit/1ba0679731eb5abde2f77871375fe2cdcb21dcc4))
* **deps:** update rust docker tag to v1.67.0 ([44755be](https://github.com/cailloumajor/frontend-utils-wasm/commit/44755be7d0f1a58d80c6fb4fc8da909715c90789))
* wrongly reverted logic of identify_last iterator ([74b2151](https://github.com/cailloumajor/frontend-utils-wasm/commit/74b2151ab60e61d4d5b31e62afc99ab029fe9e87))

## [2.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.4.1...v2.0.0) (2023-01-19)


### ⚠ BREAKING CHANGES

* rename the project

### Bug Fixes

* rename the project ([bfcd637](https://github.com/cailloumajor/frontend-utils-wasm/commit/bfcd6377b645e95df9ee41430a8750822f3da7c6))

## [1.4.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.4.0...v1.4.1) (2023-01-15)


### Miscellaneous Chores

* release 1.4.1 ([c399170](https://github.com/cailloumajor/frontend-utils-wasm/commit/c39917042dfcc6f0c29c6206370bfe20ba5565ea))

## [1.4.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.3.0...v1.4.0) (2023-01-02)


### Features

* allow null color in InfluxDB data ([5145b5b](https://github.com/cailloumajor/frontend-utils-wasm/commit/5145b5b61f9bf33333f09a5dd4819ffee97ed4bd))

## [1.3.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.2.0...v1.3.0) (2022-12-22)


### Features

* add opacity option ([b96847d](https://github.com/cailloumajor/frontend-utils-wasm/commit/b96847d2821c46050dcea462f744f9750e31eb27))


### Bug Fixes

* adjust margins to show labels in all cases ([ea8391b](https://github.com/cailloumajor/frontend-utils-wasm/commit/ea8391b93695dca9cdbbde796c3b10f21b4d5dbe))

## [1.2.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.1.1...v1.2.0) (2022-12-20)


### Features

* use CSS color property to draw the mesh ([c08cbb8](https://github.com/cailloumajor/frontend-utils-wasm/commit/c08cbb84c0152638d8f74f4ddf678a1a33153bfc))

## [1.1.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.1.0...v1.1.1) (2022-12-20)


### Bug Fixes

* allow drawing the last rectangle ([04f4d6e](https://github.com/cailloumajor/frontend-utils-wasm/commit/04f4d6e2d9a2512320c16c613763378bda437250))
* **deps:** update rust crate serde to 1.0.151 ([421ede1](https://github.com/cailloumajor/frontend-utils-wasm/commit/421ede1600faf1139e3bbddf6ebdbd07e8fddd4d))
* **deps:** update rust crate thiserror to 1.0.38 ([3fe0f25](https://github.com/cailloumajor/frontend-utils-wasm/commit/3fe0f25ab10bef844ead5cb67780864a5dcec788))

## [1.1.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v1.0.0...v1.1.0) (2022-12-16)


### Features

* switch to a more consistent object design ([f04048e](https://github.com/cailloumajor/frontend-utils-wasm/commit/f04048e685c0e57c2bb72286e9ff6c1129686133))

## [1.0.0](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.5...v1.0.0) (2022-12-16)


### Bug Fixes

* **deps:** update rust crate wasm-bindgen to 0.2.83 ([3463d30](https://github.com/cailloumajor/frontend-utils-wasm/commit/3463d30de8899e33702579c542263757fab96a71))
* set missing Accept header ([7f04f61](https://github.com/cailloumajor/frontend-utils-wasm/commit/7f04f611301a3c87e4df910c38525180f981633e))
* **tests:** move request tests to Cypress ([0f2ad2e](https://github.com/cailloumajor/frontend-utils-wasm/commit/0f2ad2e8de2bebe59ef0e034513b68784dafaf56))


### Miscellaneous Chores

* release 1.0.0 ([87685b5](https://github.com/cailloumajor/frontend-utils-wasm/commit/87685b5286e212c42a0b218a33438fbc8c3978f6))

## [0.1.5](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.4...v0.1.5) (2022-12-15)


### Bug Fixes

* **ci:** set package scope ([78fe5db](https://github.com/cailloumajor/frontend-utils-wasm/commit/78fe5dbfe1dcee7bbeb63cf48a35e4352b96b0f3))

## [0.1.4](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.3...v0.1.4) (2022-12-15)


### Bug Fixes

* **ci:** give package path to npm publish ([220bcb8](https://github.com/cailloumajor/frontend-utils-wasm/commit/220bcb83bdf6248ec5926788d0959bdfdc6a2030))

## [0.1.3](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.2...v0.1.3) (2022-12-15)


### Bug Fixes

* **ci:** publish with npm ([59f73dd](https://github.com/cailloumajor/frontend-utils-wasm/commit/59f73dd2c3998c2e5f5d96a0aaa6b2acac8f5c3f))

## [0.1.2](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.1...v0.1.2) (2022-12-15)


### Bug Fixes

* **ci:** set always-auth ([0d10d2a](https://github.com/cailloumajor/frontend-utils-wasm/commit/0d10d2a4b1fc8740e0da26995188bbb5d357ed4f))

## [0.1.1](https://github.com/cailloumajor/frontend-utils-wasm/compare/v0.1.0...v0.1.1) (2022-12-15)


### Bug Fixes

* **ci:** add missing wasm-pack install ([be98b2e](https://github.com/cailloumajor/frontend-utils-wasm/commit/be98b2e0f4dc40dfdb7575a66003aea67eaac804))

## 0.1.0 (2022-12-15)


### Features

* add font family option ([844005e](https://github.com/cailloumajor/frontend-utils-wasm/commit/844005ecb2716c7b97892e58826ef1d0bb40aee2))
* initial implementation ([985f27b](https://github.com/cailloumajor/frontend-utils-wasm/commit/985f27b6af7de9d5ac889b8c4b10583589f96215))
