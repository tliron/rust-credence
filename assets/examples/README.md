Examples
========

Use [generate-self-signed-keys.sh] to regenerate keys if necessary.

* [Hello World](hello-world/): Single Markdown page, default configuration (no configuration file)
* [Blog](blog/): An elaborate example showcasing Jinja templates, custom error pages, catalogs, TLS, and JavaScript
* [Multi](multi/): Shows how to serve multiple domains with TLS (see note below)
* [Defaults](defaults/): Full `credence.yaml` configuration file with all the defaults and some documentation.

We use self-signed keys for TLS, so your browser won't be happy with them. You'll need to accept them as an exception. With `curl` use the `--insecure` flag.

For the multi example you need the domain names `site1` and `site2` to exist for the client. A simple way to do this locally is to add something like this to your `/etc/hosts` file:

```
::1 site1
::1 site2
```

You can then try these URLs: [http://site1:8000](http://site1:8000), [http://site2:8000](http://site2:8000) (we set up redirection to the "https" port).
