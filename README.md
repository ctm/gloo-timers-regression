# Gloo Timers 0.2.3 regression

It looks like gloo-timers 0.2.3 introduded a regression.  I couldn't reproduce
this under trunk, although perhaps I didn't spend enough time trying.

## How to reproduce

```
$ yarn install
$ yarn dev:start
```

Look on at http://localhost:8080 and open up the JavaScript console.  Under
gloo-timers 0.2.3, you'll probably see a panic.  You won't see that panic
under 0.2.2.


