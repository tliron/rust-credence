Credence: Basics Guide
======================

Quickstart
----------

First, [get it](https://github.com/tliron/rust-credence/releases).

> We currently provide binaries for Linux, specifically with a relatively recent version of [glibc](https://sourceware.org/glibc/). If that's not what you have then you can build Credence yourself on your target machine by [getting Rust](https://www.rust-lang.org/tools/install) and then running `cargo install credence`. If building locally is impossible or doesn't work for your platform then you can always run our binary in a container. Note that it's also possible to build for Linux using [musl](https://musl.libc.org/) instead of glibc.

Then run `credence`, pointing it at one (or more) directories of site assets. You can start with the [examples](../assets/examples) included in the download package or this repository. For example (with extra verbose logs):

```
git clone https://github.com/tliron/rust-credence.git
credence -vv rust-credence/assets/examples/blog
```

You should now be able to see the site locally at [`http://localhost:8000`](http://localhost:8000) and follow the log messages in the terminal.

Credence will look for a configuration file at `.credence/credence.yaml` in your site assets directory. If it doesn't find the file then it will use sensible defaults. [Here's](../assets/examples/defaults/.credence/credence.yaml) an example configuration file that has all the documentation and defaults.

See the [advanced guide](advanced.md) guide for more information about serving multiple sites from the same process.


Your URL Space
--------------

You may be familiar with web servers that straightforwardly map file paths to URL paths. Credence works the same way, but with a few differences.

First, it insists on *no trailing slashes* for any URLs. By default it will redirect URLs with trailing slashes to those without them. This makes things a bit more awkward for managing your relative links, but it's much nicer for human users.

Second, it hides all `.html` file suffixes. These extensions are an implementation detail that human users really don't have to see. For example, the `articles/my-article.html` file will be mapped to the `articles/my-article` URL.

Actually, we also hide `.r.md` and `.r.yaml` midfix extensions. More on that below.

Third, it maps directories using the `index.html` *or* `index.r.*` file in them. This is basically what most web servers do, just that Credence adds support for that "r" midfix file. For example, the `articles/index.html` file will be mapped to the `articles` URL.

Note that the consequence is that you can create the same URL in one of two ways:

* The `hobbies.html` file maps to the `hobbies` URL
* The `hobbies/index.html` file *also* maps to the `hobbies` URL

Also note that `index.*` file is *not* mapped. It can only be accessed via the directory URL. Likewise, hidden files and directories (those beginning with a `.`) are also not mapped.


Markdown for Authoring
----------------------

What makes Credence fun is that you can author your content in simple [Markdown](https://en.wikipedia.org/wiki/Markdown). To enable Markdown for a page, insert the `.r` midfix before the `.md` file extension, so: `.r.md`.The "r" stands for "rendered".

Credence will hide the whole suffix when mapping to a URL. So the `my-article.r.md` file would map to the `my-article` URL.

> Why this awkward midfix? Because it allows us to keep the `.md` extension, which makes it easier to open the file with an editor and have it recognized as Markdown. The midfix allows us to differentiate rendered pages from non-rendered ones, thus you can serve `.md` files as is without Credence rendering them, should you need to.

By default Credence expects [GitHub-Flavored Markdown](https://github.github.com/gfm/). However, you can also use plain [CommonMark](https://commonmark.org/) if you prefer. There's a good guide for it [here](https://www.markdownguide.org/).

Let's try it! Create the file `my-artice.r.md` with this content:

```markdown
Hello, World!
=============

* One
* And two
* And ... four! Tricked you!
```

Now you can see [`http://localhost:8000/my-article`](http://localhost:8000/my-article) in your local browser. Nice.

Markdown does not render into complete HTML pages, so you might be wondering where the rest of the HTML comes from. It's from a Jinja template (see below), into which Credence inserts your rendered Markdown. If you haven't created any templates of your own, Credence will use a simple "just works" default one.

### Caching

Note that it can take up to 5 seconds for your page to be updated for users from the moment you modify it, because that's the default cache duration. Read more about caching in the [advanced guide](advanced.md).

### JSON

If you send a `GET` to the page's URL with an `Accept: application/json` header then you will get the JSON with same variables that Jinja gets. You can make use of this endpoint with JavaScript to fetch and render the page dynamically with cool widgets. If you want the JSON indented for improved human readabiliy, add the `pretty=true` query to the URL. Command line example:

```
curl --header 'Accept: application/json' http://localhost:8000/my-article?pretty=true
```


Jinja for Design
----------------

By default Markdown pages will use the `default.jinja` HTML template. We'll learn how to change that default later on. Also by default templates will be in `.credence/templates`, so you can create a `.credence/templates/default.jinja` to change the default look.

[Jinja](https://en.wikipedia.org/wiki/Jinja_\(template_engine\)) is an easy-to-use yet powerful templating language. It was originally written in Python and is appropriately friendly. Learn about it [here](https://jinja.palletsprojects.com/en/stable/templates/).

Our implementation is MiniJinja, which has a syntax that is only slightly reduced from the Python reference. It is documented [here](https://docs.rs/minijinja/latest/minijinja/syntax/index.html). Supported filters are listed [here](https://docs.rs/minijinja/latest/minijinja/filters/index.html#functions) and [here](https://docs.rs/minijinja-contrib/latest/minijinja_contrib/filters/index.html). Credence also adds some [here](https://docs.rs/credence-lib/latest/credence_lib/render/index.html#functions). Rustaceans: It's ridiculously easy to create your own custom filters in Rust.

Credence will automatically inject some standard variables into your templates:

* `content`: This is the HTML that was rendered from Markdown.
* `title`: If not explicitly set by an annotation (see below) then Credence will grab the first heading from the Markdown content. If there is no heading then it will be an empty string.
* `created` (optional): The `created` annotation (see below). (As [Unix time](https://en.wikipedia.org/wiki/Unix_time).)
* `updated` (optional): The `updated` annotation (see below).
* `path`: The path from the URL (always begins with "/").
* `query` (optional): The query from the URL. Map of string to set of strings.
* `socket`: Information about the TCP socket through which the user request arrived. The value is a map that contains `port` (TCP port, e.g. 443), `tls` (boolean = true if TLS is enabled, i.e. "https"), and `host` (hostname used in the user URL, e.g. "mysite.org"; will be an empty string if you did not configure any hosts for the port). Checking this variable is useful for rendering variations, such as using different CSS for different domains, not showing private parts of the page if TLS is disabled, etc.
* `catalog`: This powerful variable will be explained in greater detail below.

Let's change the default look. Create the file `.credence/templates/default.jinja` with this content:

```jinja
<!DOCTYPE HTML>
<html>
    <head>
        <title>{{ title | safe }}</title>
    </head>
    <body style="color: purple;">
        {% if path != '/' %}
        <div>
            <a href="{{ path | parentpath }}">Up</a>
        </div>
        {% endif %}
        {% if created %}
        <p>Created: {{ created | dateformat }}</p>
        {% endif %}
        {{ content | safe }}
    </body>
</html>
```

Note the use of the `safe` filter for `content`: it is already in HTML and we don't want to escape it.

Now all your pages will use this template unless they specifically choose a different one via an annotation, as explained below.


Annotating Pages
----------------

Credence lets you annotate your Markdown pages in order to give you some control over their rendering as well as allowing you to add custom Jinja variables and metadata.

To add annotations, *start* your Markdown page with a [fenced code block](https://www.markdownguide.org/extended-syntax/#fenced-code-blocks), specifically using triple backticks as the delimiter. It *must* start at the very beginning of the file with no whitespace to be considered an annotation block.

By default Credence annotations are in [YAML](https://yaml.org/), which also works as a superset of JSON if you prefer that syntax. You can go ahead and add the `yaml` syntax hightlighting hint so that editors will indeed treat the code block as YAML. Nice.

> Why did we choose a fenced code block? Firstly, because Markdown intentionally does not support metadata, not even comments, we have to do *some* kind of preprocessing. What's nice about code blocks is that they are still Markdown, so the file will stay correct in any Markdown environment. Moreover, some editors support syntax hightlighting inside code blocks, giving us even that nifty feature! Secondly, because it would be quite rare to want to purposely start a Markdown file with a *rendered* code block. If you really must do so in Credence, use the triple tilde delimiter, or insert some whitespace first, or use a non-fenced code block so that Credence won't consider it an annotation block.

When Credence renders your Markdown pages it will do so *without* the annotation block. Thus it's also useful if all you want to do is add comments to your file. Just use YAML comments in the annotation block! (they begin with `#`).

Example page with annotations:

~~~markdown
```yaml
# This is my favorite page (and *this* is a YAML comment!)
template: blog.jinja
variables:
  title: Hello, World!
  rating:
    public: 7
    private: 10
```
My First Blog Post
==================

Good morning.
~~~

Here are annotations recognized by Credence:

* `created`: Explicitly set the created timestamp. For supported syntax see [dateparser](https://docs.rs/dateparser/latest/dateparser/#accepted-date-formats).
* `updated`: Set the last updated timestamp.
* `renderer`: We currently support `gfm` (GitHub Flavored Markdown, the default), or `markdown` (or `md`) for CommonMark.
* `template`: The Jinja template for this page. This is a path relative to the `.credence/templates` path. The default value is `default.jinja`.
* `variables`: This is a map of strings to extra variables that Credence will inject into the Jinja template. You can use any YAML types for the values, including maps, sequences, and nested combinations of these.
* `headers`: Set HTTP response header values (map of string to string). Note that headers don't necessarily have to go all the way to the client, as they can also be used to send hints to middleware or other HTTP components along the way. Indeed, that's how we can configure caching for the page (see the [advanced guide](advanced.md)).
* `catalog`: This powerful annotation will be explained in greater detail below.

Again, you do not *have* to add annotations if the default behavior is fine.

It's actually possible to create a file with *only* annotations. Of course you can create a `.r.md` file with no  content other that the fenced code block. However, a `.r.yaml` file will also work! This is a bit more efficient because no extra parsing is necessary, and it can be helpful if your editor recognizes `.yaml` files, which may make it easier for you to edit them.


Custom Error Pages
------------------

Want to create a custom "404 Not Found" page? What about "500 Internal Server"? (Not that that would ever happen because Credence is of course perfect and contains exactly *zero* bugs.)

Put them in the `.credence/status` directory with the name `{status code}.html`, e.g. `.credence/status/404.html`.

Note that these are *static* HTML pages, *not* Jinja templates. This is intentional, because we definitely don't want an error page to potentially cause errors itself.

You can test these via the built-in `/admin/status/{status code}` URL (see the [advanced guide](advanced.md)).


Catalogs
--------

Credence is intentionally minimalistic, but one feature we couldn't do without is automatic catalog generation.

A catalog takes all the Markdown pages in its directory and turns them into a list of items (rows), where each item has some metadata (columns) about that page. This allows you to present users with a list, table of contents, and even statistics about the catalog (e.g. the number of pages).

The following columns will always be available:

* `title`: the `title` variable, otherwise the URL slug
* `href`: the relative URL to the page from the catalog page
* `created`: the `created` variable, otherwise 0
* `updated`: the `update` variable, otherwise 0

To enable a catalog set the `catalog` annotation on any page. It's common to have it in the `index.r.md` file of a directory (or `index.r.yaml` if you don't need any Markdown content), but you can also use a different page (and URL), e.g. `catalog.r.md`. The annotation can be just an empty map to enable the defaults (`catalog: {}`) or you can set the following keys:

* `columns`: Extra columns to add beyond those listed above. Columns are taken from the `variables` annotation of each page. `columns` is a map of column names to variables. Values can simply be the variable name, e.g. `rating: rating`. You can also traverse into nested variables (maps and sequences) using a list, e.g. "`rating: [ rating, public ]`.
* `sort`: The sort column. The default is `title`, but you might want to use `created` or `updated` instead, or any of your extra columns added with `columns`. The default sort order is ascending but you can flip that by using a map value here, e.g. `sort: { column: my-value, ascending: false }`.

You can render the catalog either on the server (with Jinja) or on the client (with JavaScript):

### Server-Side Catalogs

Credence will inject the catalog as Jinja variable `catalog`, so you can simply insert it into a template:

```jinja
<ol>
{% for item in catalog %}
    <li>
        <a href="{{ item.href | safe }}">{{ item.title | safe }}</a>
    </li>
{% endfor %}
</ol>
```

(Quick tip: [List.js](https://listjs.com/) is awesome for turning the above into a nice user experience.)

### Client-Side Catalogs

Note that you don't necessarily have to render the `catalog` variable as HTML. You can also render it as JSON inside a `<script>` element and use JavaScript to handle the presentation:

```jinja
<script>
let catalog = {{ catalog | tojson(indent=2) }};
MyCoolTableDisplayLibrary.display(catalog);
</script>
```

Also, as explained above, if you send a `GET` to the catalog's URL with an `Accept: application/json` header then the resulting `catalog` key will have the entire catalog. You can thus use JavaScript to *both* fetch and render the catalog. This method can be used for accessing the catalog from other pages. For example, you can use JavaScript to display a dynamically generated navigation popup on any page of your site.
