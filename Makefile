docker.build:
	docker build -t quickest-notes-backend .

docker.run:
	docker run -p 8080:8080 quickest-notes-backend