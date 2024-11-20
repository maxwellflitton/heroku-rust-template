# heroku-rust-template
Template of rust app for being deployed onto Heroku where the JavaScript frontend is compiled into the application




```bash
heroku buildpacks:add maxwell-flitton/rust-reactjs-server
```

```bash
heroku create first-test-build
```

```bash
heroku buildpacks:add maxwell-flitton/rust-reactjs-server --app first-test-build
```


```bash
git push heroku main
```
