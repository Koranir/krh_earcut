# krh_earcut

A simple implementation of the earcut polygon triangulation algorithm, based on [this blog post](https://tchayen.github.io/posts/triangulation-of-polygons) and the [mapbox implementation](https://github.com/mapbox/earcut).

No validation is performed on input data for maximal simplicity.

## `no_std`

This crate is no_std compatible, but requires the `alloc` crate to be available.
