<?php

declare(strict_types=1);

namespace App;

/**
 * Format a string with the given arguments using sprintf.
 *
 * @param non-empty-string $template
 *
 * @pure
 */
function format(
    string $template,
    int|float|string|null ...$args,
): string {
    return sprintf($template, ...array_map(
        static fn ($arg) => is_float($arg) ? number_format($arg, 2) : $arg,
        array_filter($args, static fn ($arg) => $arg !== null)
    ));
}
