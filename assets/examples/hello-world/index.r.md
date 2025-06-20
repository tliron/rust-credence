Credence: Hello World
=====================

Hello, world!

This is the simplest example of a Credence-based web site.

It contains no configuration, just the defaults and this single page.

Just to show that Credence can serve regular files, here's an image:

![my image](image.png)

To learn more about Credence can do, check out the "blog" example.

Notes about this URL
--------------------

* This file's name without the suffixes is `index`, so Credence will serve it when a URL points to this directory. This is similar to how most web servers serve `index.html` for directories. Credence will do that, too, however it is also aware of other possible suffixes for this file. TODO hide it

Notes about this file
---------------------

* Specifically, this file's suffix is `.md`, signifying that it is in Markdown. If you open it in a good text editor it would detect that suffix and switch to Markdown editing mode.
* Moreover, it also has a `.r` *midfix*, which tells Credence that it must be rendered first (Markdown to HTML).
* Since we haven't configured any HTML templates, it will just use a simple built-in default template.

Notes about caching
-------------------

* The default cache duration is 5 seconds. Every time there is a GET to this URL, that clock is reset. Thus, if you edit this file and want to see the changes happen then 5 seconds need to pass without any GET.
* You can also force a reset to the cache:
```
curl --request POST http://localhost:8000/admin/reset-cache
```
