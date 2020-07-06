<?php

declare(strict_types=1);

$ffi = FFI::cdef(
    "int add_two_numbers(int, int);",
    __DIR__ . "/target/debug/librust_adder.so"
);

echo $ffi->add_two_numbers(3, 4) . "\n";
