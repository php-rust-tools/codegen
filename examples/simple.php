<?php

declare(strict_types=1);

namespace Foo;

use Foo\Bar\Baz as Qux;
use Throwable;
use DateTime;
use DateTimeImmutable;

use function strlen;
use function array_map;
use function array_filter;

use const Foo\HELLO;
use const Foo\HEY;


const A = "Hello World!";
const B = null;
const C = 1;
const D = false;
const E = null;
const F = 213.412;
const G = [1, 25];

/**
 * This is a simple hello function.
 *
 * @param non-empty-string $firstname
 *
 * @return string
 *
 * @pure
 */
#[Qux(foo: 1, bar: 2)]
function hello(
    #[Validation\NotBlank, Validation\Length(min: 2, max: 10)]
    string $firstname,
    string $lastname = Qux::Foo,
): string {
    return 'Hello ' . $firstname . ' ' . $lastname . '!';
}

function nothing(): void {
    // empty body
}

/**
 * This is an example class.
 *
 * @immutable
 */
abstract class Example
{
    final const A = "Hello World!";

    protected const B = null;

    private const C = 1;

    public const D = false;

    private string $foo;

    protected string $bar;

    public string|int $baz = "Hello World!";

    /**
     * This is a simple hello function.
     *
     * @param non-empty-string $firstname
     *
     * @return string
     *
     * @pure
     */
    #[Qux(foo: 1, bar: 2), Qux(foo: 1, bar: 2)]
    function hello(
        string $firstname,
        string $lastname = Qux::Foo,
    ): string {
        return 'Hello ' . $firstname . ' ' . $lastname . '!';
    }

    /**
     * This is a simple x function.
     *
     * @pure
     */
    #[Foo(foo: 1, bar: 2), Bar(foo: 1, bar: 2)]
    #[Baz, Qux]
    public function x(): mixed {
        return 'Hello!';
    }

    /**
     * This is a simple poop function.
     */
    public abstract function poop(): void;

    /**
     * This is a simple echo function.
     */
    public final function helloWorld(): void {
        echo 'Hello World!';
    }
}

