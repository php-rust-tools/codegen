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

function nothing(): void {}

function format(
    string $template,
    int|float|string|null ...$args,
): string {
    return sprintf($template, ...array_map(
        static fn ($arg) => is_float($arg) ? number_format($arg, 2) : $arg,
        array_filter($args, static fn ($arg) => $arg !== null)
    ));
}

/**
 * This is an example class.
 *
 * @immutable
 */
abstract class Example extends Foo\Bar\Baz implements Foo\Bar\BazInterface
{
    use A;
    use B, C;
    use D, E, F, G {
        E::bar as baz;
        D::foo as public bar;
        E::qux as public;
        D::format as protected;
        D::d as private;
        D::drop insteadof E;
        G::something insteadof E, F, D;
        E::e as protected;
    }

    final const A = "Hello World!";

    protected const B = null;

    private const C = 1;

    public const D = false;

    public const bool E = false;

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

class SimpleUser
{
    private string $firstName = "Jane";
    private string $lastName = "Doe";
    public string $fullname {
        get {
            return $this->firstName . ' ' . $this->lastName;
        }
        set (string $fullname) {
            [$first, $last] = explode(' ', $fullname);
            $this->firstName = $first;
            $this->lastName = $last;
        }
    }
}

class SimpleUser2
{
    private string $firstName = "Jane";
    private string $lastName = "Doe";
    public string $fullname {
        get {
            return $this->firstName . ' ' . $this->lastName;
        }
    }
}

/**
 * This is an example trait.
 */
trait ExampleTrait
{
    use A;
    use B, C;
    use D, E, F, G {
        E::bar as baz;
        D::foo as public bar;
        E::qux as public;
        D::format as protected;
        D::d as private;
        D::drop insteadof E;
        G::something insteadof E, F, D;
        E::e as protected;
    }

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

/**
 * This is an example unit enum.
 */
enum ExampleUnitEnum
{
    use A;
    use B, C;
    use D, E, F, G {
        E::bar as baz;
        D::foo as public bar;
        E::qux as public;
        D::format as protected;
        D::d as private;
        D::drop insteadof E;
        G::something insteadof E, F, D;
        E::e as protected;
    }

    /**
     * This is a foo case.
     */
    case Foo;

    /**
     * This is a bar case.
     */
    case Bar;

    case Baz;
}

/**
 * This is an example string backed enum.
 */
enum ExampleStringBackEnum: string
{
    /**
     * This is a foo case.
     */
    case Foo = "foo value";

    /**
     * This is a bar case.
     */
    case Bar = "bar value";

    case Baz = "baz value";
}

/**
 * This is a simple formatter interface.
 *
 * @immutable
 */
#[Foo(foo: 1, bar: 2), Bar(foo: 1, bar: 2)]
interface Formatter extends Qux
{
    public function format(
        string $template,
        int|float|string|null ...$args,
    ): string;
}
