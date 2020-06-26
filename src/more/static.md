# Static Content

Static content in Yew is served from the `static` directory in your crate. For example, if you have a folder structure like:

```
myapp
├── src/
│   └── lib.rs
├── static/
│   └── script.js
├── Cargo.lock
└── Cargo.toml
```

## HTML

Yew compiles to a single-page application, with one JavaScript bundle that can be imported into an HTML page. Yew generates boilerplate HTML page out of the box, but you can create a custom one in `static/index.html` if you wish to override this.

Your application will be built at `/crate.js`. For example, for your custom HTML page that imports some CSS, you might have:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>My Awesome App!</title>
  <!-- references /static/mystyles.css -->
  <link rel="stylesheet" href="/mystyles.css">
</head>
<body>
  <script src="/myapp.js"></script>
</body>
</html>
```

## CSS

A proposal for proper CSS support can be found here: [https://github.com/yewstack/yew/issues/533](https://github.com/yewstack/yew/issues/533).

For now, you can reference stylesheets be creating a custom `index.html` within the `static` directory.

