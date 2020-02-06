# mysqlpinger

A cli that pings MySQL for specified period. It is mainly intended for use when you want to check "ready" of MySQL.

## Example

```console
# wait for db started by docker-compose.
DOCKER_COMPOSE_PROJECT=xxx
DOCKER_COMPOSE_NETWORK=yyy
docker run --rm -t \
  --network=${DOCKER_COMPOSE_PROJECT}_${DOCKER_COMPOSE_NETWORK} \
  ymgyt/mysqlpinger:latest \
  --user=user --pass=secret --host=<container_name> [--forever|--max-retry=20]
```

## Usage

```console
mysqlpinger 
Ping to mysql server

USAGE:
    mysqlpinger [FLAGS] [OPTIONS] [DBNAME]

ARGS:
    <DBNAME>    Database name [env: MYSQL_DB_NAME=]  [default: sys]

FLAGS:
    -s, --silent         Running with no logging
    -v, --verbose        Verbose
        --forever        Retry without limit
        --check-slave    check slave threads status
        --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -h, --host <HOST>            MySQL server hostname [env: MYSQL_HOST=]  [default: 127.0.0.1]
    -p, --port <PORT>            MySQL server port [env: MYSQL_PORT=]  [default: 3306]
    -u, --user <USER>            User for authentication [env: MYSQL_USER=]  [default: root]
    -P, --pass <PASS>            Password for authentication [env: MYSQL_PASSWORD=]
    -m, --max-retry <COUNT>      Max retry count [default: 9]
    -i, --interval <DURATION>    Retry ping interval [default: 1s]

Example:
    # Basic
    mysqlpinger --pass=root --port=30303 <db_name>

    # Docker
    docker run --rm -t --network=<network> ymgyt/mysqlpinger:latest \
       --user=user --pass=secret --host=<container_name> [--forever|--max-retry=20]

    # Slave status check
    mysqlpinger --pass=root --port=30304 --check-slave <db_name>
```