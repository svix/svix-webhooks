# Svix PHP

## Requirements

PHP 5.6.0 and later.

## Composer

Coming Soon.

## Manual Installation

For now you can download the [latest release](https://github.com/svixhq/svix-libs/releases). Then, to use the bindings, include the `init.php` file.

```php
require_once('/path/to/svix-php/init.php');
```

## Dependencies

Svix PHP requires the following extensions in order to run:

-   [`json`](https://secure.php.net/manual/en/book.json.php)

If you use Composer, these dependencies should be handled automatically. If you install manually, you'll want to make sure that these extensions are available.

## Development

### Fmt Code

composer install
./vendor/bin/php-cs-fixer fix -v --using-cache=no .

### Running Tests

composer install
./vendor/bin/phpunit tests