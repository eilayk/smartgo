docker build -t smartgo-api .
docker run -e GO_API_KEY=your_go_api_key -p 8080:8080 smartgo-api
docker run --env-file ./.env -p 8080:8080 smartgo-api