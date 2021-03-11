I followed the Rust book to build this multi-threaded server, but some things still don't make sense to me. I list them below:

```
ab -c 100 -n 10000 http://127.0.0.1:7878/
```

When running 100 concurrent requests, I sometimes see the program locking up for ~10 seconds before continuing.

```
ab -n 100 http://127.0.0.1/sleep
```

This request to the 5-second sleep endpoint doesn't even see the sleeping behavior (each request finishes in milliseconds). Not sure why that is the case.