### Build
```cargo build```

### Test
```php -d "extension=target/debug/libGeo.dylib" -r "geo_hash_encode(11.111, 22.222);"```

```php -d "extension=target/debug/libGeo.dylib" -r "geo_hash_decode('ww8p1r4t8');"```

```php -d "extension=target/debug/libGeo.dylib" -r "geo_hash_distance('ww8p1r4t8', 'ww8p2r4t9');"```
