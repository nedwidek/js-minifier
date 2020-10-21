# js-minifier

Quick rust program for minifying a JavaScript file using javascript-minifier.com.

## Usage

js-minifier &lt;source&gt; &lt;output&gt;

## Building

At its simplest:

* cargo build --release

However, you might get a pkg_config error on Linux for OpenSSL. This is easily solved by:

* apt-get install openssl-dev (Ubuntu)
* dnf install openssl-devel (CentOS)
