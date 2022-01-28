# Can't call clear_timeout from webpack

This demo illustrates the surprising behavior where calling the
"global" clearTimeout panics (and hence doesn't clear the timeout),
but calling the "global" setTimeout works fine.

This illustrates why [commit f5761bf15cc3f12d77046290e76242eebbac0500](https://github.com/rustwasm/gloo/commit/f5761bf15cc3f12d77046290e76242eebbac0500#diff-e8dbc4ed3de3d04f8d6d9d375736801aab2521d783b7e58e5952c981293b8eb9) broke
gloo-timers under webpack.

FWIW, I read the [source to
js_sys::global](https://docs.rs/js-sys/latest/src/js_sys/lib.rs.html#5389-5464)
and the [globalThis MDN Web
Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis)
and it's not at all clear to me that either `setTimer` or `clearTimer`
"should" work globally without going through `global()`, however, it's
undeniable that they both work under trunk, but `clearTimer` fails
under webpack.

## How to reproduce

```
$ yarn install
$ yarn start:dev
```

Visit http://localhost:8080 and open up the JavaScript console. The output will
be
```
No death yet (the first timer was cleared)
gloo-timers-web.js:1479 Uncaught (in promise) TypeError: Illegal invocation
    at __wbg_clearTimeout_42a8676f07d366c5 (gloo-timers-web.js:1479:85)
    at gloo_timers::clear_timeout::h77aceca93d460608 (gloo-timers-web.wasm:0xe78d)
    at gloo_timers::run_app::h98f0af0fa6fb5339 (gloo-timers-web.wasm:0x3464)
    at run_app (gloo-timers-web.wasm:0xaa97)
...
index_bg.js:338 This timer was not cleared
```

This shows that if `js_sys::global()` is used to get the "global
object", `clearTimer` (and presumably `setTimer`) can be called as a
method and that will work under trunk and webpack.  I don't know
enough about node to be sure, considering [the
discussion](https://github.com/rustwasm/gloo/pull/185) that went into
the commit that changed things, my guess is it, or something similar,
would work.
