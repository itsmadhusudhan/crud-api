# Crud Api

## Taskone - task apis
Apis to create and manage tasks

### Get All Tasks
```bash
curl -X GET \
  http://localhost:8080/api/v1/tasks
```

### Create Task
```bash
curl -X POST \
  http://localhost:8080/api/v1/tasks \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Task 1",
    "description": "Task 1 description",
    "dueDate": "2024-01-01T00:00:00.000Z"
}'
```

### Get Task
```bash
curl -X GET \
  http://localhost:8080/api/v1/tasks/1
```

### Update Task
```bash
curl -X PUT \
  http://localhost:8080/api/v1/tasks/1 \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Task 1 updated",
    "description": "Task 1 description updated"
}'
```

### Delete Task
```bash
curl -X DELETE \
  http://localhost:8080/api/v1/tasks/1
```


