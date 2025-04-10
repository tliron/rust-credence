Credence: User Guide
====================

Quickstart
----------

First, [get it](https://github.com/tliron/rust-credence/releases).

You can also build it yourself using `cargo install credence`.

Then, run it, pointing it at a directory of site assets. You can start with the [examples](assets/examples) included in this repository. For example (with extra verbose logs):

```
git clone https://github.com/tliron/rust-credence.git
credence -vv rust-credence/assets/examples/hello-world
```

You should now be able to see the site locally at [`http://localhost:8080`](http://localhost:8080) and follow the log messages in the terminal.

Credence will look for a configuration file in `.credence/credence.yaml`. If it doesn't find it, it will use defaults. We go into configuration in detail in the [configuration guide](CONFIGURATION.md).

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

What makes Credence fun is that you can author your content in simple [Markdown](https://en.wikipedia.org/wiki/Markdown). To do this, insert the `.r` midfix before the `.md` file extension, so: `.r.md`.The "r" stands for "rendered".

Credence will hide the whole suffix when mapping to a URL. So the `my-article.r.md` file would map to the `my-article` URL.

> Why this awkward midfix? Because it allows us to keep the `.md` extension, which makes it easier to open the file with an editor and have it recognized as Markdown. The midfix allows us to differentiate rendered pages from non-rendered ones, thus you can serve `.md` files as is without Credence rendering them, should you need to.

By default Credence expects [GitHub Flavored Markdown](https://github.github.com/gfm/). However, you can also use plain [CommonMark](https://commonmark.org/) if you prefer. There's a good guide [here](https://www.markdownguide.org/).

Let try it! Create the file `my-artice.r.md` with this content:

```markdown
Hello, World!
=============

* One
* And two
* And ... four! Tricked you!
```

Now you can see [`http://localhost:8080/my-article`](http://localhost:8080/my-article) in your browser. Nice.

Markdown does not render into complete HTML pages, so you might be wondering where the rest of the HTML comes from. It's from a Jinja template! Credence inserts your rendered Markdown into one. If you haven't included any templates of your own, Credence will use a simple default one.

### JSON

If you send a `GET` to the page's URL with an `Accept: application/json` header then you will get the JSON of all the values. You can thus use JavaScript to fetch and render it dynamically.

Jinja for Design
----------------

By default Markdown pages will use the `default.jinja` HTML template. We'll learn how to change that default later on. And by default templates will be in `.credence/templates`, so you can create a `.credence/templates/default.jinja` to change the default look.

[Jinja](https://en.wikipedia.org/wiki/Jinja_\(template_engine\)) is an easy-to-use yet powerful templating language. It was originally written in Python and is suitably friendly. Learn about it [here](https://jinja.palletsprojects.com/en/stable/templates/).

Our implementation is MiniJinja, which has a syntax that is only slightly reduced. It is documented [here](https://docs.rs/minijinja/latest/minijinja/syntax/index.html). Supported filters are listed [here](https://docs.rs/minijinja/latest/minijinja/filters/index.html#functions) and [here](https://docs.rs/minijinja-contrib/latest/minijinja_contrib/filters/index.html).

Credence will automatically inject some standard variables into your templates:

* `content`: This is the HTML that was rendered from Markdown.
* `title`: If not explicitly set by an annotation (see below) then Credence will grab the first heading from the Markdown content.
* `created`: The date the page was first created. If not explicitly set by an annotation (see below) then Credence will use the creation timestamp of the file.
* `updated` (optional): The date the page was last updated. Credence does *not* set this automatically from file, because we don't want every file edit to count as an update for users.
* `catalog` (optional): This powerful variable will be explained in greater detail below.

Let's change the default look. Create the file `.credence/templates/default.jinja` with this content:

```jinja
<!DOCTYPE HTML>
<html>
    <head>
        <title>{{ title | safe }}</title>
    </head>
    <body style="color: purple;">
        <p>Created: {{ created }}</p>
        {{ content | safe }}
    </body>
</html>
```

Note the use of the `safe` filter. That's because `content` is already in HTML and we don't want to escape it.

Now all your pages will use this template unless they specifically choose a different one via an annotation, as explained below.

Page Annotations
----------------

Credence lets you annotate your Markdown pages in order to give you some control over their rendering as well as allowing you to add custom metadata.

To add annotations, *start* your Markdown page with a [fenced code block](https://www.markdownguide.org/extended-syntax/#fenced-code-blocks), specifying using triple backticks as the delimiter. It *must* start at the very beginning of the file with no whitespace to be considered an annotation block.

> Why did we choose a fenced code block? First, because Markdown intentionally does not support metadata, not even comments, we have to do *some* kind of preprocessing. What's nice about code blocks is that they are still Markdown, so the file will stay correct in any Markdown environment. Moreover, some editors support syntax hightlighting, giving us even that nifty feature! Second, because it would be quite rare to want to purposely start a Markdown file with a *rendered* code block. If you really must do so in Credence, insert some whitespace first or use a non-fenced code block so that Credence won't consider it an annotation block.

By default Credence annotations are in [YAML](https://yaml.org/), which also works as a superset of JSON if you prefer that syntax. You can go ahead and add the `yaml` syntax hightlighting hint so that editors will indeed treat the code block as YAML. Nice. Example page with annotations:

~~~markdown
```yaml
# This is a YAML comment
template: blog.jinja
values:
  title: Hello, World!
  rating:
    public: 7
    private: 10
```
My First Blog Post
==================

Good morning.
~~~

When Credence renders your Markdown pages it will do so *without* the annotation block. Thus, it's useful if all you want to do is add comments to your file. Just use YAML comments in the annotation block! (they begin with `#`).

Here are annotations recognized by Credence:

* `created`: Explicitly set the created timestamp. If it's not present, Credence will use the modification timestamp of the file. (Credence uses [dateparser](https://docs.rs/dateparser/latest/dateparser/#accepted-date-formats) for all timestamps.)
* `updated`: Set the last updated timestamp.
* `renderer`: We currently support `gfm` (GitHub Flavored Markdown, the default), or `markdown` (or `md`) for CommonMark.
* `template`: The Jinja template for this page. This is a path relative to the `.credence/templates` path. The default value is `default.jinja`.
* `values`: This is a map of strings to extra values that Credence will inject into the Jinja template. You can use any YAML types for the value, including maps, sequences, and nested combinations of these.
* `headers`: Specifically set HTTP response header values (map of string to string). Note that headers don't necessarily have to go all the way to the browser, as they can also be used to send hints to middleware along the way, such as our [caching/encoding middleware](https://docs.rs/kutil-http/latest/kutil_http/tower/caching/struct.CachingLayer.html).
* `catalog`: This powerful annotation will be explained in greater detail below.

Again, you do not *have* to add annotations if the default behavior is fine.

It's actually possible to create a file with *only* annotations. Of course you can create a `.r.md` file with no  content other that the fenced code block. However, a `.r.yaml` file will also work! This is a bit more efficient because no extra parsing is necessary, and it can be helpful if your editor recognizes `.yaml` files and make it easier for you to work with them.

Catalogs
--------

Credence is intentionally minimalistic, but one feature we couldn't do without is automatic catalog generation.

A catalog takes a directory of Markdown pages and turns it into a list of items (rows), where each item has some metadata (columns) about a page. This allows you to present users with a list, table of contents, and even statistics about the catalog (e.g. the number of pages).

The following columns will always be available:

* `title`
* `href`: the relative URL to the page
* `created`: as [Unix time](https://en.wikipedia.org/wiki/Unix_time)
* `updated`: also as Unix time, but will be 0 if not available

To enable a catalog set the `catalog` annotation on any page. It's common to have it in the `index.r.md` file of a directory (or `index.r.yaml` if you don't need any Markdown content). The annotation can be just an empty map for the defaults (`catalog: {}`) or you can set the following keys:

* `columns`: Extra columns to add beyond those listed above. Columns are taken from the `values` annotation of each page. `columns` is a map of column names to values. Values can simply be the value name, e.g. `rating: rating`. You can also traverse into nested values (maps and sequences) using a list, e.g. "`rating: [ rating, public ]`.
* `sort`: The sort column. The default is `title`, but you might want to use `created` or `updated` instead, or any of your extra columns added with `columns`. The default sort order is ascending but you flip that by using a map value here, e.g. `sort: { column: my-value, ascending: false }`.

Credence supports both server-side and client-side access to the catalog:

### Server-Side Catalogs

Easy! Credence will inject the catalog as a Jinja value, so you can simply insert it into a template:

```jinja
<ol>
{% for item in catalog %}
    <li>
        <a href="{{ item.href | safe }}">{{ item.title | safe }}</a>
    </li>
{% endfor %}
</ol>
```

Note that you don't necessarily have to render it into HTML. You can, for example, render it as JSON inside a `<script>` element and use JavaScript to handle the presentation:

```jinja
<script>
let catalog = {{ catalog | tojson(indent=2) }};
MyCoolTableDisplayLibrary.display(catalog);
</script>
```

### Client-Side Catalogs

Also easy! As explained above, if you send a `GET` to the catalog's URL with an `Accept: application/json` header then the resulting `catalog` key will have the entire catalog. You can thus use JavaScript to *both* fetch and render the catalog.

This can be used for accessing the catalog from other pages. For example, you can use JavaScript to display a dynamically generated navigation popup on any page of your site.

Coordinating File Modifications
-------------------------------

Credence will properly handle conditional HTTP. This powerful optimization feature allows browsers to cache content locally. When sending a request to your server, they will let it know the timestamp they have in their cache. If the server has a newer version, it will send it (and the client will update its cache). Otherwise, it will send a [304 status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/304), which tells the browser that it's safe to use their cached content.

Conditional HTTP improves the user experience and saves bandwidth and compute resources for both the client and the server.

That said, it can be tricky for the server to determine the "last modified" timestamp. Credence's starting point is the modification timestamp of the file. However, Jinja templates introduce a problem: If the template that a Markdown page relies on has been changed, how can we apply that information to the file? Also consider that Jinja templates often include other templates, and can do so dynamically, so we're actually dealing with a dependency tree.

One possible solution is to follow that file's dependency tree, accumulate all modification timestamps, and use the most recent of them. However, doing so would require us to follow through most of the rendering codepath. Thus, even though we might end up saving bandwidth and client resources, we're not saving much on *server* compute resources. It's an unsatisfying solution.

What Credence does instead is use a single empty file as the "coordinator" and have us associate any number of files and directories (recursively) with it. Any modifications made to the coordinated files will also update the coordinator's modification timestamp (Credence achieves this via listening to operating system notifications). Thus the coordinator will always have the most recent modification timestamp of all the coordinated files.

The modification timestamp for an individual page is calculated as the most of recent between its own file's and that of the coordinator. The result is that any change to the coordinated files will cause browsers to invalidate their cache.

By default Credence will coordinate the `.credence/templates` directory (recursively). Thus if you modify *any* template file then browsers will be sure to get the most recent versions of pages. You can remove this coordination or add other files and directories (see the [configuration guide](CONFIGURATION.md)).

Easy and simple!

The downside is that it can lead to unnecessary page invalidations. For example, you might be modifying a template that only *some* pages use, but our solution will cause *all* pages to be invalidated. This, however, we deem a small price to pay for a simple solution that guarantees cache invalidation. Add to this that template modifications would likely be infrequent. You'll mostly be authoring content, not changing you're site's basic design.

### Modifying Loaded Files

What if you modify a `.css` file? You'd want browsers to invalidate their cache for it, too, so that they will get the latest version. But if they think that the main page hasn't changed then won't try to re-fetch the `.css` file. The same goes for images included in the `.css`, `.js` files, and any other files loaded by your pages *after* they themselves are loaded.

The solution is simply to add these files to the coordinator. When they change, the page will be invalidated, and the browser will try to fetch them again.

### Modifications and Caching

TODO

systemd
-------

Credence works easily with [systemd](https://systemd.io/), and can even log directly to its [journald](https://www.freedesktop.org/software/systemd/man/latest/systemd-journald.service.html) service.

See the [example service file](assets/systemd/credence.service). To follow the logs:

```
journalctl --follow --unit=credence
```

Configuration
-------------

You're a Credence expert now! Well, almost. Continue with the [configuration reference](CONFIGURATION.md)).
