Credence: Advanced Guide
========================

Make sure to check out the [example configuration file](../assets/examples/defaults/.credence/credence.yaml) that has all the defaults and some documentation.


Built-in Administrative URLs
----------------------------

Credence has some useful administrative URLs:

* `/admin/about`: Provides credence-lib build information as text
* `/admin/status/{status code}`: Simulates a status code, which is useful to testing your custom error pages (e.g. 404)
* `/admin/reset-cache` (POST): Invalidates all pages in the (server-side) cache (more about caching below).
* `/admin/shutdown` (POST): Initiates a graceful shutdown of the Credence process. If you're using some kind of service manager (e.g. systemd, see below) then this may trigger a restart.

You probably don't want to expose these URLs to all your visitors! To protect them, use `protect` in your `.credence.yaml`:

```yaml
urls:
  protect:
  - regex: ^/admin
    username: generalissimo
    password: mypassword
```

You can also use `hide` instead if you prefer to disable them entirely.

To send a POST you can use the command line, for example:

```
curl --request POST http://generalissimo:mypassword@localhost:8000/admin/reset-cache
curl --request POST http://generalissimo:mypassword@localhost:8000/admin/shutdown
```


Serving Multiple Sites
----------------------

If your sites run on different TCP ports then you *could* just run multiple Credence processes.

Unfortunately, this is not possible if they share a port. The most straightforward reason is that operating systems will only let one process bind to a port at a time.

A common solution is to run each site on a different port and then use a reverse proxy (e.g. Nginx) to route according to the host name.

However, Credence provides a built-in alternative. Indeed, you can serve multiple sites from the same process simply by listing more than one site directory in the command line. Each site will have its configuration file. The only requirement is that you *must* specify `hosts` in the configuration for any shared port, and (logically) a specific host for a specific port can only be used once. You will get a configuration error if there is an overlap.

> Under the hood there is some sophistication in making this all work. Beyond just routing requests to the correct site, Credence also has to ensure that each host can have its own TLS key. This must happen at the low level of TLS session management.


Server-Side Caching and Compression
-----------------------------------

Server-side caching is vital for scalable web sites. It's also useful as part of a strategy of defense against denial-of-service attacks.

Credence's caching mechanism is especially efficient in that it incorporates compression (a.k.a. encoding). Credence will compress responses according the client preferences, supporting the common web compression algorithms: Brotli, Zstandard, Gzip, and Deflate. The efficiency concern is that even though compression saves bandwidth, it does use up compute resources on the server. By integrating compression into caching, Credence makes sure to only ever compress responses *once* per compression algorithm as long as the response is still in the cache.

> Rustaceans! Credence's caching layer is available separately as general-purpose Tower middleware. See the [kutil](https://docs.rs/kutil-http/latest/kutil/http/tower/caching/struct.CachingLayer.html) library.

By default, Credence will cache responses in memory for 5 seconds. While a URL's page is in the cache, disk files will *not* be accessed and Markdown and Jinja will *not* be rendered. The response will be sent purely from memory.

You can change the default duration in `credence.yaml` and you can also set it per page via the `XX-Cache-Duration` header in annotations. For example:

~~~
```yaml
headers:
  XX-Cache-Duration: 10 minutes
```
This page will be cached for 10 minutes! Wow!
~~~

You can disable caching entirely for a page by setting `XX-Cache` to `false`. Note that these custom headers will *not* pass through to clients.

> Obviously, larger cache durations will lead to better results, so why didn't we set the default duration to "1 day" or even "1 year"? We decided on a small cache duration by default because we assume most users will want to be able to make live changes to their site's assets and have clients to see those changes (almost) immediately. However, if you understand the consequences, you can definitely increase that default. You could still make live changes, but you would then have to invalidate the cache manually via the `/admin/reset-cache` URL (see above) or more brutally by restarting Credence.

Your `credence.yaml` also has an `encoding` section to configure compression. By default it disables compression for certain already-compressed media types, namely videos and music files, as well as for very small files for which the bandwidth savings may not be worth the compute effort. You can also set the `XX-Encode` header to `false` to disable compression per page.


Client-Side Caching
-------------------

Credence will properly handle conditional HTTP. This powerful optimization feature allows clients to cache content locally.

This is how it works: When sending a request to your server, clients will let it know the timestamp of the data they have in their cache. If your server has a newer version, it will send it back and the client will update its cache with the new content and new timestamp. Otherwise (if there's no new content) your server will send a [304 status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status/304) and an empy response body, which tells the client that it's safe to use their cached content.

Conditional HTTP makes the user experience smoother and saves bandwidth and compute resources for *both* the client *and* the server. It is a critical scalability tool.

> Note that client-side caching is *not* a replacement for server-side caching. For one, it only works for one client at a time. But also, it's optional. You cannot expect all your clients to support it, especially malicious clients that want to purposely waste your resources.

As powerful as it is, it can be tricky to implement. Specifically, servers need a way to determine that "last modified" timestamp. Credence's starting point is the modification timestamp of the file. However, the use of Jinja templates introduces a challenge: If the template that a Markdown page relies on has been changed, how can we apply that information to the HTTP response? Also consider that Jinja templates often include other templates, and can do so dynamically, so we're actually dealing with a potentially sprawling dependency tree.

One possible solution is to follow each file's dependency tree, accumulate all modification timestamps, and use the most recent of all of them. However, doing so would require us to follow through most of the rendering codepath. Thus, even though we would save bandwidth and client resources, we're not saving much on *server* compute resources. It's an unsatisfying solution.

> Actually, conditional HTTP supports [ETags](https://en.wikipedia.org/wiki/HTTP_ETag) as an alternative to using timestamps. But it's not a good fit for Credence.

### Coordinating File Modifications

What Credence does instead is designate a single empty file as the "coordinator" for the site. You can then configure any number of files and directories (recursively) that will be coordinated with it. Any modifications made to the coordinated files will cause an update to the coordinator's modification timestamp, thus the coordinator will always have the most recent modification timestamp of all the coordinated files.

The modification timestamp for an individual page is calculated as the most recent between its own file's and that of the coordinator. Thus a modification of *any* coordinated files will cause clients to invalidate their caches.

By default Credence will coordinate the `.credence/templates` directory (recursively) so if you modify *any* template file then clients will be sure to get the most recent versions of pages. You can change this behavior or add other files and directories in your `credence.yaml` configuration.

> Don't forget that client-side caching *follows* the server-side cache. If you modify a file you still have to wait for cache entries to expire before clients will stop getting 304 codes.

### Forced Invalidation

Credence handles coordination by listening to operating system notifications, so this feature only works while Credence is running. The consequence is that the coordinator can be out of date if modifications happened while Credence was not running. To guarantee that this would not happen Credence always updates the coordinator when it starts up, meaning that a restart will cause client invalidation.

Of course, because it's just a file (by default it's `.credence/.coordinator`), you can force invalidation at will simply by "touching" it to update its timestamp:

```
touch .credence/.coordinator
```

### False Positives?

The downside of this solution that it can lead to unnecessary page invalidations (false positives). For example, you might be modifying a template that only *some* pages use, but Credence's coordination will cause *all* pages to be invalidated.

This, however, we deem this a small price to pay for a simple solution that guarantees cache invalidation. Add to this that template modifications would likely be infrequent. You'll mostly be authoring content, not changing your site's basic design.

### Coordinating Resource Files

What if you modify a `.css` file? You'd want clients to invalidate their cache for it, too, so that they will get the latest version. But if they think that the containing page hasn't changed then won't try to re-fetch the `.css` file. The same goes for images included in the `.css`, as well as `.js` files, and any other resource files loaded by your pages *after* they themselves are loaded.

The solution is simply to add these files to the coordinator. If you modify them then the containing pages will be invalidated causing clients to get new versions of those pages which will in turn cause them to attempt to re-fetch the resources. It's a bit of a roundabout, but easy to achieve.

It's common, for example, to place all such files in a single directory (we like to name it `_`), so you can just add it in `credence.yaml` in addition to `templates`, and bob's your uncle:

```yaml
coordinate:
  paths:
  - templates
  - ../_
```
