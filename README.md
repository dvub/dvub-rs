cargo watch -x run -w src -w assets -w pages

# for tailwind hot reload
bunx tailwindcss -i ./pages/input.css -o ./assets/output.css --watch