Kinto Rust client
#################

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

Use cargo::

  $ cargo build


Run tests
=========

In one terminal, run a Kinto server:

::

    $ make runkinto

In another, run the tests against it:

::

    $ cargo tests
