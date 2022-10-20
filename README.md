# badpod

A Rust crate for interpreting imperfect feeds of podcasts.

## Motivation

On a backend server, which has to communicate with a database, strict schemas are typically used.
Therefore, if content from an external source that does not conform to the schema is loaded, one may fail to deserialize that content successfully.

This scenario is very common in podcasting space, where RSS feeds of podcasts may  
- combine multiple standards (namespaces)
- have some elements missing
- have some values in wrong data types
- etc.

In that case, we may want a more flexible, intermediate schema.
A schema that does *not* throw an error the instant it encounters an unexpected value but rather one that **deserializes the content that it is able to** and **stores the content that it failed to deserialize** for further cleanup.
