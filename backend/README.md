
后端工作空间执行 `cargo add actix-web async-graphql rbatis` 后的输出, 说明了哪些功能是默认启用的, 哪些功能没有默认启用.

```shell
Adding actix-web v4.0.1 to dependencies.
    Features:
    + __compress
    + actix-http/compress-brotli
    + actix-http/compress-gzip
    + actix-http/compress-zstd
    + actix-macros
    + actix-web-codegen
    + compress-brotli
    + compress-gzip
    + compress-zstd
    + cookie
    + cookies
    + macros
    - actix-tls
    - experimental-io-uring
    - openssl
    - rustls
    - secure-cookies
Adding async-graphql v3.0.38 to dependencies.
    Features:
    - apollo_persisted_queries
    - apollo_tracing
    - blocking
    - bson
    - cbor
    - chrono
    - chrono-duration
    - chrono-tz
    - dataloader
    - decimal
    - futures-channel
    - futures-timer
    - hashbrown
    - iso8601-duration
    - log
    - lru
    - opentelemetry
    - password-strength-validator
    - rust_decimal
    - secrecy
    - serde_cbor
    - sha2
    - smol_str
    - string_number
    - time
    - tracing
    - tracing-futures
    - unblock
    - url
    - uuid
    - zxcvbn
Adding rbatis v3.1.6 to dependencies.
    Features:
    + all-database
    + default_mode
    + rbatis-core
    + rbatis-core/all-database
    + rbatis-core/runtime-tokio-rustls
    + rbatis-macro-driver
    + rbatis_sql
    + runtime-tokio-rustls
    - debug_mode
    - format_bson
    - mssql
    - mysql
    - postgres
    - runtime-actix-native-tls
    - runtime-actix-rustls
    - runtime-async-std-native-tls
    - runtime-async-std-rustls
    - runtime-tokio-native-tls
    - sqlite
    - upper_case_sql_keyword
Adding async-graphql-actix-web v3.0.38 to dependencies.
    Features:
    - cbor
    - serde_cbor     
Adding actix-rt v2.7.0 to dependencies.
    Features:
    + actix-macros
    + macros
    - io-uring
    - tokio-uring     
Adding serde v1.0.136 to dependencies.
    Features:
    + std
    - alloc
    - derive
    - rc
    - serde_derive
    - unstable     
Adding env_logger v0.9.0 to dependencies.
    Features:
    + atty
    + humantime
    + regex
    + termcolor
Adding log v0.4.16 to dependencies.
    Features:
    - kv_unstable
    - kv_unstable_serde
    - kv_unstable_std
    - kv_unstable_sval
    - max_level_debug
    - max_level_error
    - max_level_info
    - max_level_off
    - max_level_trace
    - max_level_warn
    - release_max_level_debug
    - release_max_level_error
    - release_max_level_info
    - release_max_level_off
    - release_max_level_trace
    - release_max_level_warn
    - serde
    - std
    - sval
    - value-bag   
Adding lazy_static v1.4.0 to dependencies.
    Features:
    - spin
    - spin_no_std  
Adding dotenv v0.15.0 to dependencies.
    Features:
    - clap
    - cli        
Adding rbson v2.0.3 to dependencies.
    Features:
    - chrono-0_4
    - serde_with
    - uuid-0_8  
Adding dotenv v0.15.0 to dependencies.
    Features:
    - clap
    - cli
Adding lazy_static v1.4.0 to dependencies.
    Features:
    - spin
    - spin_no_std      
```