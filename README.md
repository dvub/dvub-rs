cargo watch -x run -w src -w assets -w templates

# for tailwind hot reload
bunx tailwindcss -i ./templates/input.css -o ./assets/output.css --watch