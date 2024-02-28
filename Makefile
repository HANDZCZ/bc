run:
	docker compose up -d

build: build-backend build-frontend build-database

build-backend:
	cd backend && build_musl.cmd
	docker build -t handz/bc-backend:latest ./backend

build-frontend:
	cd frontend && trunk build --release
	docker build -t handz/bc-frontend:latest ./frontend

build-database:
	docker build -t handz/bc-database:latest ./database

run-clean-build: stop-and-delete-db-data build run

run-clean: stop-and-delete-db-data run

stop-and-delete-db-data:
	docker compose down -v

stop:
	docker compose down

recreate: stop run

nuke: stop-and-delete-db-data

push-backend:
	docker image push handz/bc-backend:latest

push-frontend:
	docker image push handz/bc-frontend:latest

push-database:
	docker image push handz/bc-database:latest

push: push-backend push-frontend push-database