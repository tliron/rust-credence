Credence: Tips
==============

Authoring History
-----------------

Credence purposely doesn't keep a history of file edits as this feature is much better handled by mature, specialized software.

We recommend [Git](https://git-scm.com/).

By the way, in case you didn't know this: To use Git you do *not* need to set up an account on a public repository server (e.g. [GitHub](https://github.com/)), nor do you need to set up your own server. You can work entirely with clones in the filesystem, which you can then backup as regular old files. It's powerful and easy.


Remote Authoring
----------------

Your Credence website is likely being served by a desktop-less server. Of course, you can do all your authoring from the terminal over ssh, mosh, etc. There are many great terminal text editors with excellent built-in Markdown and Jinja support. That said, you might be more comfortable with GUIs.

There are various ways to use a GUI with remote files, but by far the most straightforward solution is to mount your remote site's files to your local filesystem over ssh so that your local GUI tools would be able to work with your remote files just as if they were local:

* [sshfs](https://github.com/libfuse/sshfs) for Linux
* [sshfs-win](https://github.com/winfsp/sshfs-win) for Windows
* [macFUSE](https://macfuse.github.io/) for macOS

Some GUIs we like:

* [MarkText](https://github.com/marktext/marktext)
* [Git Cola](https://github.com/git-cola/git-cola)


https and http
--------------

You really should be using TLS ("https") to protect everyone's privacy. Services such as [Let's Encrypt](https://letsencrypt.org/) provide browser-approved keys for free. (For an internal site, you can use self-signed keys and even manage your own certificate authority.)

That said, web browsers (and humans) still default to the (outdated) "http://" as the prefix for URLs.

Credence makes it easy to accept "http://" URLs and force a redirect your secured site. Here's a `credence.yaml` example snippet:

```yaml
ports:
  80:
    name: http
    hosts:
    - host: mysite.org
      redirect-to: 443
  443:
    name: https
    hosts:
    - host: mysite.org
      key:
        certificates: { path: /etc/letsencrypt/live/mysite.org/fullchain.pem }
        private-key: { path: /etc/letsencrypt/live/mysite.org/privkey.pem }
```


systemd
-------

Credence can be trivially set up to be managed by [systemd](https://systemd.io/), and can even log directly to the [journald](https://www.freedesktop.org/software/systemd/man/latest/systemd-journald.service.html) service.

See our [example service file](../assets/systemd/credence.service), which you can place in `/usr/lib/systemd/system/`. In this basic setup we purposely isolate the service to run as user `credence`, but do note that non-root users cannot normally bind to ports 80 and 443 (anything below 1024). One simple way to get this permission is to grant it to the `credence` executable itself:

```
sudo setcap 'cap_net_bind_service=+ep' /usr/bin/credence
```

To follow the journald logs:

```
journalctl --follow --unit=credence.service
```
