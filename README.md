# nba_database

## 启动DB musql
```
docker pull mysql:lastest
```

```
docker run -d --name nba-data \
  --mount type=bind,source=__dir/database,target=/var/lib/mysql \
  -e MYSQL_ROOT_PASSWORD=qwer1234 \
  mysql
```