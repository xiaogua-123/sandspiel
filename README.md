<meta charset="utf-8"/>

# sandspiel · 落沙游戏 · 砂遊び · Песочная игра · Jeu de sable

> "Imagine the cool phenomenon when the wind blows the falling leaves. This game simulates the phenomenon with powder (dots)!" — DAN-BALL

![](Screenshot.png)

[English](#english) | [中文](#中文) | [日本語](#日本語) | [한국어](#한국어) | [Русский](#русский) | [Français](#français)

---

## English

A [falling sand](https://en.wikipedia.org/wiki/Falling-sand_game) game with **136 elements** built in Rust (via WASM), WebGL, and JavaScript. Features a level system with goals, fluid simulation, and a Tauri desktop app.

[Play Online](https://sandspiel.club) | [Project Write-up](https://maxbittker.com/making-sandspiel)

### Features
- 136 element types: metals, crystals, powders, liquids, gases, creatures, explosives, magic, and more
- 10 levels with clear objectives (eliminate targets, create elements)
- Real-time fluid simulation (water, lava, acid, blood...)
- Sandbox mode for free creation
- Upload and share your creations
- Desktop app via Tauri (macOS / Windows / Linux)

### Build
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# Watch mode (separate terminal):
cargo watch -s 'wasm-pack build'
```

Fluid simulation adapted from [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation) by PavelDoGreat.

---

## 中文

一款拥有**136种元素**的[落沙游戏](https://en.wikipedia.org/wiki/Falling-sand_game)，使用 Rust (WASM)、WebGL 和 JavaScript 构建。包含关卡目标系统、流体模拟和 Tauri 桌面应用。

[在线游玩](https://sandspiel.club) | [项目介绍](https://maxbittker.com/making-sandspiel)

### 特色
- 136种元素：金属、晶体、粉末、液体、气体、生物、爆炸物、魔法等
- 10个带通关目标的关卡（消灭目标、创造元素）
- 实时流体模拟（水、岩浆、酸液、血液...）
- 自由沙盒模式，随意创作
- 上传和分享你的作品
- Tauri 桌面应用（macOS / Windows / Linux）

### 构建
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# 监视模式（另一个终端）：
cargo watch -s 'wasm-pack build'
```

流体模拟代码改编自 [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation)。

---

## 日本語

**136種類**の元素を持つ[落砂ゲーム](https://en.wikipedia.org/wiki/Falling-sand_game)。Rust（WASM）、WebGL、JavaScriptで構築。レベル目標システム、流体シミュレーション、Tauriデスクトップアプリを搭載。

[オンラインで遊ぶ](https://sandspiel.club) | [プロジェクト詳細](https://maxbittker.com/making-sandspiel)

### 特徴
- 136種類の元素：金属、結晶、粉末、液体、気体、生物、爆発物、魔法など
- クリア目標付きの10レベル（ターゲット排除、元素生成）
- リアルタイム流体シミュレーション（水、溶岩、酸、血液...）
- 自由に創作できるサンドボックスモード
- 作品のアップロードと共有
- Tauriデスクトップアプリ（macOS / Windows / Linux）

### ビルド
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# 監視モード（別ターミナル）：
cargo watch -s 'wasm-pack build'
```

流体シミュレーションは [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation) を改変。

---

## 한국어

**136가지** 원소가 포함된 [떨어지는 모래 게임](https://en.wikipedia.org/wiki/Falling-sand_game). Rust(WASM), WebGL, JavaScript로 제작. 레벨 목표 시스템, 유체 시뮬레이션, Tauri 데스크톱 앱 탑재.

[온라인 플레이](https://sandspiel.club) | [프로젝트 소개](https://maxbittker.com/making-sandspiel)

### 특징
- 136가지 원소: 금속, 결정, 분말, 액체, 기체, 생물, 폭발물, 마법 등
- 목표가 있는 10개 레벨 (제거 목표, 원소 생성)
- 실시간 유체 시뮬레이션 (물, 용암, 산, 피...)
- 자유로운 샌드박스 모드
- 작품 업로드 및 공유
- Tauri 데스크톱 앱 (macOS / Windows / Linux)

### 빌드
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# 감시 모드 (별도 터미널):
cargo watch -s 'wasm-pack build'
```

유체 시뮬레이션은 [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation)을 기반으로 함.

---

## Русский

[Игра «Падающий песок»](https://en.wikipedia.org/wiki/Falling-sand_game) со **136 элементами**, созданная на Rust (WASM), WebGL и JavaScript. Включает систему уровней с целями, симуляцию жидкостей и десктопное приложение на Tauri.

[Играть онлайн](https://sandspiel.club) | [Описание проекта](https://maxbittker.com/making-sandspiel)

### Возможности
- 136 типов элементов: металлы, кристаллы, порошки, жидкости, газы, существа, взрывчатка, магия и другое
- 10 уровней с целями (уничтожение целей, создание элементов)
- Симуляция жидкостей в реальном времени (вода, лава, кислота, кровь...)
- Режим песочницы для свободного творчества
- Загрузка и публикация своих работ
- Десктопное приложение Tauri (macOS / Windows / Linux)

### Сборка
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# Режим отслеживания (отдельный терминал):
cargo watch -s 'wasm-pack build'
```

Симуляция жидкости адаптирована из [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation).

---

## Français

Un [jeu de sable tombant](https://en.wikipedia.org/wiki/Falling-sand_game) avec **136 éléments**, construit en Rust (WASM), WebGL et JavaScript. Inclut un système de niveaux avec objectifs, une simulation de fluides et une application de bureau Tauri.

[Jouer en ligne](https://sandspiel.club) | [Article du projet](https://maxbittker.com/making-sandspiel)

### Fonctionnalités
- 136 types d'éléments : métaux, cristaux, poudres, liquides, gaz, créatures, explosifs, magie, etc.
- 10 niveaux avec objectifs (éliminer des cibles, créer des éléments)
- Simulation de fluides en temps réel (eau, lave, acide, sang...)
- Mode bac à sable pour création libre
- Téléverser et partager vos créations
- Application de bureau Tauri (macOS / Windows / Linux)

### Compilation
```bash
cd crate && wasm-pack build && cd ..
npm install --legacy-peer-deps
npm run start

# Mode surveillance (terminal séparé) :
cargo watch -s 'wasm-pack build'
```

Simulation de fluides adaptée de [WebGL-Fluid-Simulation](https://github.com/PavelDoGreat/WebGL-Fluid-Simulation).

---

### Credits
Original by [Max Bittker](https://maxbittker.com). Enhanced with 112 additional elements by the community.
