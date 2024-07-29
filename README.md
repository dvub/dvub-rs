

cargo watch -x run -w src -w assets -w templates 

bunx tailwindcss -i ./templates/input.css -o ./assets/output.css --watch