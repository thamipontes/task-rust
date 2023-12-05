# task-rust
Create and List yout tasks!

# Docker
After downloadin the application on your machine, you will need run docker-compose. If you don't have Docker, you can download it via the links below:

- Windows:
    - https://docs.docker.com/docker-for-windows/install/
- Linux:
  - https://docs.docker.com/engine/install/ubuntu/
- Mac:
  - https://docs.docker.com/docker-for-mac/install/

Docker-compose:
- https://docs.docker.com/compose/install/

# Run 
Once you have Docker and Docker-compose installed, simply run the command below in the root of the project:
```bash
docker-compose up -d
```
## Pgadmin
With Pgadmin you will be able to see the data that is being saved in the database.
- Access the link http://localhost:5050/
- Log in with the username and password below:
  - User: pgadmin4@pgadmin.org
  - Password: admin
- Right click on Servers and select Create > Server
- In the General tab, enter the server name as "Tasks"
- In the Connection tab, enter the data below:
    - Host name/address: host.docker.internal
    - Port: 5433
    - Maintenance database: tasks
    - Username: postgres
    - Password: root

### Create a Task

- Step 1: Use the curl below

```bash
curl --location 'http://localhost:8080/register' \
--header 'Content-Type: application/json' \
--header 'Cookie: JSESSIONID=D1D746CC044C01430F4F46F63CC5F2EF' \
--data '{
    "name": "Make a cake"
}'
```

- Step 2: Use pgadmin to check if the data was saved in the database

### List all Tasks

- Step 1: Use the curl below

```bash
curl --location 'http://localhost:8080/tasks' \
--header 'Cookie: JSESSIONID=D1D746CC044C01430F4F46F63CC5F2EF'
```