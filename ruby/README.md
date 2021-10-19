# Svix Ruby

Please refer to [the documentation](https://docs.svix.com) for usage instructions.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'svix'
```

And then execute:

```sh
bundle
```

Or install it yourself as:

```sh
gem install svix
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).


### Run Tests

bundle exec rspec spec


### Build & Upload

```sh
gem build svix.gemspec

gem push pkg/svix-<VERSION>.gem
```
