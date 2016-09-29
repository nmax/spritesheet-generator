lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'rusty_sprite/version'

Gem::Specification.new do |spec|
  spec.name = 'rusty_sprite'
  spec.version = RustySprite::VERSION
  spec.authors = ['Maximilian Neger']
  spec.email = ['maximilian.neger@nix-wie-weg.de']

  spec.summary = 'DSL for Rusty-Sprite'
  spec.description = 'Small wrapper arround an commandline spritesheet' \
                     'generator'
  spec.extensions = %w(rust/extconf.rb)

  # # Prevent pushing this gem to RubyGems.org by setting 'allowed_push_host', or
  # # delete this section to allow pushing this gem to any host.
  # if spec.respond_to?(:metadata)
  #   spec.metadata['allowed_push_host'] = "TODO: Set to 'http://mygemserver.com'"
  # else
  #   raise 'RubyGems 2.0 or newer is required to protect against public gem pushes.'
  # end

  spec.files = `git ls-files -z`.split("\x0").reject do |f|
    f.match(%r{^(test|spec|features)/})
  end

  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  spec.add_development_dependency 'bundler', '~> 1.11'
  spec.add_development_dependency 'rake', '~> 10.0'
  spec.add_development_dependency 'rspec', '~> 3.0'
  spec.add_development_dependency 'byebug'
  spec.add_development_dependency 'pry-byebug'
end
