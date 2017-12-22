/*!
Using `env_logger`.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL = 'info'
```

Also try setting the `RUST_LOG_STYLE` environment variable to `0` to disable colors:

```no_run,shell
$ export RUST_LOG_STYLE = 0
```
*/

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init_from_env("MY_LOG_LEVEL");

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}
