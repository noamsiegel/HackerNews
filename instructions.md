# Goal
Edit the front end of the Tauri project to be a list of the top stories from Hacker News using the Hacker news api.

# Requirements
- the UI must be a scrollable list of Hacker News top stories
- It must show the title of the story
- The hacker news api should be fetched with a rust function and then passed to the front end
- Each hacker news item in the home page should be clickable to display the full story

# Tools to use
- Tauri
- Svelte
- Hacker News API
- Rust

# File Structure

.
├── README.md
├── node_modules
├── package-lock.json
├── package.json
├── src
│   ├── app.html
│   └── routes
├── src-tauri
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── build.rs
│   ├── capabilities
│   ├── gen
│   ├── icons
│   ├── src
│   ├── target
│   └── tauri.conf.json
├── static
│   ├── favicon.png
│   ├── svelte.svg
│   ├── tauri.svg
│   └── vite.svg
├── svelte.config.js
├── tsconfig.json
└── vite.config.js