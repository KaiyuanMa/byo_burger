# byo_burger

Build Your Own Burger API to practice TDD and backend fundamentals

## Start Service

```
cargo run
```

## Run Tests

```
API_SCHEME=http API_HOST=localhost:8080 cargo test
```

get: http://localhost:8080/ingredients

post http://localhost:8080/burgers
params: {
"bun": String,
"patty": String,
"cheese": String,
"toppings": Array,
"sauces": Array,
}
