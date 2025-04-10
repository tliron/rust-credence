Credence: Advanced Guide
========================

Built-in Administrative URLs
----------------------------

Credence enables some useful URLs by default:

* `/admin/status/{status code}`: Simulates a status code, which is useful to testing your custom error pages (see below).
* `/admin/reset-cache` (POST): Invalidates all pages in the (server-side) cache.
* `/admin/shutdown` (POST): Initiates a graceful shutdown of the Credence process. If you're using some kind of service manager (e.g. systemd; see below) then this may trigger a restart.

You probably don't want to expose these URLs to all your users! To protect them, use `protect` in your `.credence.yaml`:

```yaml
uri:
  protect:
  - regex: ^/admin
    username: generalissimo
    password: mypassword
```

You can also use `hide` if you prefer to disable them.

To send a POST you can use the command line, for example:

```
curl --request POST http://generalissimo:mypassword@localhost:8000/admin/reset-cache
curl --request POST http://generalissimo:mypassword@localhost:8000/admin/shutdown
```

Coordinating File Modifications
-------------------------------

Credence will properly handle conditional HTTP. This powerful optimization feature allows browsers to cache content locally. When sending a request to your server, they will let it know the timestamp they have in their cache. If the server has a newer version, it will send it and the client will update its cache with the new content and timestamp. Otherwise, the server will send a [304 status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/304), which tells the browser that it's safe to use their cached content.

Conditional HTTP makes the user experience smoother and saves bandwidth and compute resources for *both* the client and the server and is a critical scalability tool.

However, it can be tricky for servers to determine the "last modified" timestamp. Credence's starting point is the modification timestamp of the file. However, the use of Jinja templates introduces a challenge: If the template that a Markdown page relies on has been changed, how can we apply that information to the HTTP response? Also consider that Jinja templates often include other templates, and can do so dynamically, so we're actually dealing with a dependency tree.

One possible solution is to follow that file's dependency tree, accumulate all modification timestamps, and use the most recent of all of them. However, doing so would require us to follow through most of the rendering codepath. Thus, even though we would save bandwidth and client resources, we're not saving much on *server* compute resources. It's an unsatisfying solution.

What Credence does instead is use a single empty file as the "coordinator" and have us associate any number of files and directories (recursively) with it. By default it's at `.credence/.coordinator`. Any modifications made to the coordinated files will also update the coordinator's modification timestamp (Credence achieves this via listening to operating system notifications). Thus the coordinator will always have the most recent modification timestamp of all the coordinated files.

The modification timestamp for an individual page is calculated as the most of recent between its own file's and that of the coordinator. Thus any change to the coordinated files will cause browsers to invalidate their caches.

By default Credence will coordinate the `.credence/templates` directory (recursively). Thus if you modify *any* template file then browsers will be sure to get the most recent versions of pages. You can remove this coordination or add other files and directories (see the [configuration guide](CONFIGURATION.md)).

Easy and simple!

The downside is that it can lead to unnecessary page invalidations. For example, you might be modifying a template that only *some* pages use, but our solution will cause *all* pages to be invalidated. This, however, we deem a small price to pay for a simple solution that guarantees cache invalidation. Add to this that template modifications would likely be infrequent. You'll mostly be authoring content, not changing you're site's basic design.

Note that you can force invalidation simply by touching the coordinator file, e.g.:

```
touch .credence/.coordinator
```

### Modifying Loaded Files

What if you modify a `.css` file? You'd want browsers to invalidate their cache for it, too, so that they will get the latest version. But if they think that the main page hasn't changed then won't try to re-fetch the `.css` file. The same goes for images included in the `.css`, `.js` files, and any other files loaded by your pages *after* they themselves are loaded.

The solution is simply to add these files to the coordinator. When they change, the page will be invalidated, and the browser will try to fetch them again.

### Modifications and Caching

TODO

TLS (Transport Layer Security)
------------------------------

Credence can serve multiple domains, each with their own TLS keys, on the same port.

So you don't need a reverse proxy.

Install your own.

Or fetch using [ACME (Automatic Certificate Management Environment)](https://en.wikipedia.org/wiki/Automatic_Certificate_Management_Environment) from a provider.

Popular free one is [Let's Encrypt](https://letsencrypt.org/).

systemd
-------

Credence works easily with [systemd](https://systemd.io/), and can even log directly to its [journald](https://www.freedesktop.org/software/systemd/man/latest/systemd-journald.service.html) service.

See the [example service file](assets/systemd/credence.service). To follow the logs:

```
journalctl --follow --unit=credence
```
