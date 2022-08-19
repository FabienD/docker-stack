# The docker Stack

This project is composed of a collection of usefull docker-compose files and a cli tool to manage them.

---

## A collection of docker-compose files

A pre-configured docker-compose files [collection](collection) helping web developers.

The collection is composed of five categories (web, data, logging, tools, monitoring). To use a service, each ```.dist``` files need to be copy as as ```.yml``` file and customized.

| Application     | Description             | Category      | container name | Image docker | Documentation |
|-----------------|-------------------------|------------|----------------|--------------|---------------|
| Traefik | expose localy your application through a local domain | web | stack.web.reverse | [⤴](https://hub.docker.com/_/traefik) | [⤴](https://docs.traefik.io/en/latest/) |
| Mail Catcher | catch all emails sent | web | stack.web.mailcatcher | [⤴](https://hub.docker.com/r/schickling/mailcatcher) | [⤴](https://mailcatcher.me/) |
| PostgreSQL | A relational database | data | stack.data.postgres | [⤴](https://hub.docker.com/_/postgres/) | [⤴](https://www.postgresql.org/docs/) |
| MySQL | A relational database | data | stack.data.mysql | [⤴](https://hub.docker.com/_/mysql) | [⤴](https://dev.mysql.com/doc/) |
| Redis | The cache | data | stack.data.redis | [⤴](https://hub.docker.com/_/redis) | [⤴](https://redis.io/docs/) |
| RabbitMQ| The message broker | data | stack.data.rabbitmq | [⤴](https://hub.docker.com/_/rabbitmq) | [⤴](https://www.rabbitmq.com/documentation.html) |
| Rsyslog | A log aggregator | logging | stack.logging.rsyslog | [⤴]() | [⤴]() |
| Loki | A log aggregator | logging | stack.logging.loki | [⤴](https://hub.docker.com/r/grafana/loki) | [⤴](https://grafana.com/docs/loki/latest/?pg=oss-graf&plcmt=quick-links) |
| Grafana | The dashboard | tools | stack.tools.grafana | [⤴](https://hub.docker.com/r/grafana/grafana) | [⤴](https://grafana.com/docs/grafana/latest/?pg=oss-graf&plcmt=quick-links) |
| Portainer CE | The container manager | tools | stack.tools.portainer | [⤴](https://hub.docker.com/r/portainer/portainer.ce) | [⤴](https://docs.portainer.io/) |

You can add our own services to the collection, decide which services are enabled by default or not.
Softwares version, credentials are also configurable, this is defined in the environment file on the root of the collection.

Don't hesitate to make a PR to improve the collection.

## Requirements

Docker is required to run the stack. The stack share the same [docker network](https://docs.docker.com/network/) to facilated communication between services. The network is named "stack_dev" in all ```docker-compose.yml``` file.

```bash
docker network create stack_dev
```

Be free to change the name of the network or use multiple networks.

### 1. Web

This is the base of all web projects, composed of the following services :

- A reversed proxy service (Traefik), which is a reverse proxy service that forwards requests to your different applications. You don't have to worry about the port number, the request can be routed to the right application by following a local domain name rule (a host rule in Traefik).
- A mail catcher service (MailCatcher), which is a service that catches all emails sent from your applications.

#### 1.1. Reverse proxy

The reverse proxy web interface is exposed through a local domain. The domain is defined in the environnement file in the root of the docker-compose files collection.
Copy the ```.env.dist``` file to ```.env```, and change the domain to your local domain.

https://dashboard.stack.local (default)

![Traefil Dashboard](doc/assets/stack_reverse_proxy.png)

#### 1.2. MailCatcher

As the reverse proxy service, the mail catcher service is also exposed through a local domain.

http://mailcatcher.stack.local (default)

![Mailcatcher Dashboard](doc/assets/stack_mailcatcher.png)

### 2. Data

- A Redis service, which is a service that provides a cache.
- A RabbitMQ service, which is a service that provides a message broker.
- A PostgreSQL service, which is a service that provides a database.
- A MySQL service, which is a service that provides a database.

#### 2.1. Redis

Redis service is exposed on port 6379.

#### 2.2. RabbitMQ

RabbitMQ service is exposed on port 5672, and accessible at http://rabbitmq.stack.local for management.

![RabbitMQ Dashboard](doc/assets/stack_rabbitmq.png)

#### 2.3. PostgreSQL

PostgreSQL service is exposed on port 5432, the default port for PostgreSQL.

#### 2.4. MySQL

MySQL service is exposed on port 3306, the default port for MySQL.

### 3. Logging

- A Loki service (Loki), which is a service that provides a log viewer.
- A log service (Rsyslog), which is a service that provides a log aggregator.

#### 3.1. Loki

Loki API service is exposed on port 3100, the default port for Loki. Loki is use to collected Docker logs of containers where you have selected Loki as driver.

To use Loki, you need to [install the docker driver](https://grafana.com/docs/loki/latest/clients/docker-driver/) before, create the loki configuration using the sample file in ```logging/docker/loki/loki-local-config.yaml```. Then, connect loki API to your Grafana

Loki metrics are accessible at http://loki.stack.local/metrics.

We use Grafana (in the tools stack) to explore loki collected logs. [Signin in Grafana](http://grafana.stack.local) with the default credentials (admin/admin) and add a Loki datasource. [Add a new datasource](http://grafana.stack.local/datasources), and select Loki as type.

Use the Loki container name as URL : http://stack.logging.loki:3100

![Loki Datasource](doc/assets/stack_loki.png)

#### 3.2. Rsyslog

WIP, not yet provided.

### 4. Monitoring

WIP, not yet provided.

### 5. Tools

- A container manager, which is a tool to manage containers (Portainer).
- A Grafana service (Grafana), which is a service that provides a dashboard.

#### 5.1. Portainer

Portainer is a service that provides a web interface to manage containers.

http://portainer.stack.local (default)

![Portainer Dashboard](doc/assets/stack_portainer.png)

#### 5.2. Grafana

As you have seen if you use Loki, Grafana is also used to explore data collected by different types of sources. You can add a new data source, and select the type of data you want to explore. We use Grafana to visualize tracking and logging data.

## A cli tools to register and manage docker-compose files

The cli tools source can be found at [cli](./cli/), it is a Rust binary.

### The cli goals

The cli tool can "manage" multiple docker-compose files (start, stop, restart, ...) from everywhere in your terminal.
All registered docker-compose files are stored in a configuration file (config.toml), by default in your home directory (~/.config/dctl/config.toml).

### The config file

The config file is a TOML file, with the following structure :

```toml
[main]
DOCKER_PATH = "/usr/bin/docker"

[collections]

[collections.stack_web]
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/web/docker-compose.yml",
]

[collections.stack_logging]
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/logging/docker-compose.yml",
]

[collections.stack_tools]
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/tools/docker-compose.yml",
]

[collections.stack_data]
enviroment_file = "/home/fabien/workspace/infra/docker-stack/.env"
compose_files = [
    "/home/fabien/workspace/infra/docker-stack/data/docker-compose.yml",
]

[collections.porject_name1]
enviroment_file = "/home/fabien/workspace/apps/project1/.env"
compose_files = [
    "/home/fabien/workspace/apps/project1/docker-compose.yml",
]

[collections.porject_name2]
enviroment_file = "/home/fabien/workspace/apps/project2/.env"
compose_files = [
    "/home/fabien/workspace/apps/project2/worker/docker-compose.yml",
    "/home/fabien/workspace/apps/project2/api/docker-compose.yml",
]
```


### The cli usage

#### List registered docker-compose files

```bash
dctl list
```

#### Start a docker-compose file

```bash
dctl start -n <name>
```

#### Stop a docker-compose file

```bash
dctl stop -n <name>
```

#### Restart a docker-compose file

```bash
dctl restart -n <name>
```

