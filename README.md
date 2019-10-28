RUST_BACKTRACE=1  systemfd --no-pid -s http::3000 -- cargo watch -x run

apollo client:codegen --endpoint='http://127.0.0.1:8080/graphql' --target=typescript --watch
