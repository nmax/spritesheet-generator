# encoding: utf-8
require 'yaml'

module RustySprite
  class Generator
    def initialize(config_path)
      @sprites = YAML.load_file(config_path).map do |key, config|
        files = config['source'].reduce([]) do |all_files, glob_path|
          all_files + Dir.glob("app/assets/#{glob_path}")
        end.map { |p| File.absolute_path(p) }

        { name: config['name'] || key,
          scss_out: File.absolute_path("app/assets/#{config['scss_out']}"),
          img_out: File.absolute_path("app/assets/#{config['img_out']}"),
          files: files }
      end
    end

    # TODO: Das könnte auch ein ffi Aufruf werden.
    def build
      bin =
        "#{File.dirname(__FILE__)}/../../rust/target/release/sprite-generator"

      @sprites.each do |sprite|
        next if sprite[:files].empty?
        system(bin,
               '--name', sprite[:name],
               '--scss-out', sprite[:scss_out],
               '--image-out', sprite[:img_out],
               *sprite[:files])
      end
    end
  end
end
