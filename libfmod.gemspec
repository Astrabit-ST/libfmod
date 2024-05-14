# frozen_string_literal: true

require_relative "lib/libfmod"

Gem::Specification.new do |spec|
  spec.name = "libfmod"
  spec.version = "#{(FMOD::VERSION >> 16).to_s(16)}.#{(FMOD::VERSION >> 8 & 0xFF).to_s(16)}.#{(FMOD::VERSION & 0xFF).to_s(16)}"
  spec.authors = ["Speak2Erase"]
  spec.email = ["melody@nowaffles.com"]

  spec.summary = "FMOD Ruby bindings"
  spec.description = spec.summary
  spec.homepage = "https://github.com/Astrabit-ST/libfmod"
  spec.required_ruby_version = ">= 2.6.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libfmod/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
