> create container `docker run -it --rm --name postgres-cn -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres` or using docker-compose

> `docker exec -it postgres-cn -bash`

> `su - postgres` & `psql`

```sql
\d ticket;
```

```sql
SELECT * FROM ticket;
```

---

## Docker compose

> `docker-compose up -d`

> `docker-compose down`
