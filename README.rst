Kinto Rust client
#################

.. image:: https://img.shields.io/crates/v/kinto_http.svg
    :target: https://crates.io/crates/kinto_http

.. image:: https://img.shields.io/travis/Kinto/kinto-http.rs.svg
    :target: https://travis-ci.org/Kinto/kinto-http.rs



Kinto is a service that allows users to store and synchronize
arbitrary data, attached to a user account. Its primary interface is
HTTP.

*kinto-http* is a Rust library that eases the interactions with a
*Kinto* server instance. A project with related goals is also
available `for JavaScript <https://github.com/kinto/kinto-http.js>`_
and `for Python <https://github.com/kinto/kinto-http.js>`_.


Installation
============

You can add kinto-http to your dependencies with cargo.
To get the latest release::

    [dependencies]
    kinto_http = "0.1.0"


Contributing
============

Fist, clone this Github repository. You can use cargo to build the library::

    $ cargo build


Running examples
----------------

You can add and run existing examples on the `examples/` directory and run them with::

    $ cargo run --example <example_name>

.. note::

    The example name should omit the .rs extension.


Running tests
-------------

Run a Kinto server in background::

    $ pip install kinto
    $ kinto start --ini config.ini

Then run the tests using cargo in a single thread::

    $ RUST_TEST_THREADS=1 cargo tests --verbose
