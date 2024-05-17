<?php

echo "Encoded: " . geo_hash_encode(11.111, 22.222, 5) . "\n\n";

echo "Decoded:\n";
print_r(geo_hash_decode('s5zykv6u9141'));
echo "\n";

echo "Distance: " . geo_hash_distance('ww8p1r4t8', 'ww8p2r4t9') . "\n\n";