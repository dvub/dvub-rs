# dvub-rs
My personal website, built with Rust, Axum, Tera, HTML, and Tailwind CSS. (No JS frameworks!)

I have to give huge credit to the following repos for inspiration

- https://github.com/Bechma/todo-axum-askama-htmx
- https://github.com/jacob-ian/rs-htmx

This is mostly just for me, but if you want to clone my website and work on it, you can use these commands:

For the backend:
```
cargo install cargo-shuttle
# for development:
cargo shuttle run
# after pushing changes, to update hosting: 
cargo shuttle deploy
```

For the frontend: 
```
npm install # or whatever package manager you want
npm run dev # or `run prod` for minified css
```