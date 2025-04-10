* Built-in [ACME](https://en.wikipedia.org/wiki/Automatic_Certificate_Management_Environment) client that can auto-renew your TLS keys (no need for [Certbot](https://certbot.eff.org/)!)

Automagical TLS
---------------

Credence has a built-in [ACME (Automatic Certificate Management Environment)](https://en.wikipedia.org/wiki/Automatic_Certificate_Management_Environment) client that fetches the TLS key from a provider and also attempts to renew it when it expires.

A popular free ACME provider is [Let's Encrypt](https://letsencrypt.org/).

Note that this client uses ACME's TLS-ALPN-01 [challenge type](https://letsencrypt.org/docs/challenge-types/), which allows fetching the keys from the same port used to serve the site (usually 443). If your provider only supports other challenge types (e.g. HTTP-01) then you must use an external client, such as [Certbot](https://certbot.eff.org/), instead.
