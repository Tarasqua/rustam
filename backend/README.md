```sh
docker run --name rustam -e POSTGRES_PASSWORD=110320 -d -p 5432:5432 postgres
```

```sh
# sqlx migrate add create_task_table 
sqlx database create
sqlx migrate run
```

```sh
source .env
```

```sh
RUST_LOG=trace cargo run
```

```sh
curl -X POST "http://localhost:30303/task" \
    -d '{"title": "Task one", "description": "my first task"}' \
    -H "Content-Type: application/json" \
    | jq 

curl -X GET "http://localhost:30303/tasks" | jq 

curl -X PUT "http://localhost:30303/task/1" \
    -d '{"title": "updated", "description": "update"}' \
    -H "Content-Type: application/json" \
    | jq

curl -X DELETE "http://localhost:30303/task/1" \
    -H "Content-Type: application/json" \
    | jq
```
