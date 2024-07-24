# postgrids

An (very work in progress) attempt at building a postgres extension for working with British and Irish national grids (OSGB, and OSI). Provides a simple interface for converting valid grid references into eastings / northings and vice versa, as well as functionality to recalculate a grid reference to a new precision.

Built using the amazing [pgrx](https://github.com/pgcentralfoundation/pgrx), and wraps functionality from [gridish](https://github.com/BIS-Brecon/gridish).

Currently, does not support converting gridrefs into Geospatial primitives, as pgrx does not support postgis.

## Examples

```sql
select osgb_from_string('SO892437');
---
SO892437

select osgb_from_eastings_northings(389200, 243700, 100);
---
SO892437

select osgb_precision('SO892437');
---
100

select osgb_recalculate('SO892437', 1000);
---
SO8943
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.