```console
$ docker run -d -p 9411:9411 openzipkin/zipkin

$ cd ./examples/opentelemetry-zipkin
$ cargo run

$ firefox http://localhost:9411/
```
