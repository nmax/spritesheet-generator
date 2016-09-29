# encoding: utf-8
require 'yaml'

module RustySprite
  class Generator
    def initialize(config)
      @config = parse_config(config)
    end

    # TODO: Das könnte auch ein ffi Aufruf werden.
    def build
      bin =
        "#{File.dirname(__FILE__)}/../../rust/target/release/sprite-generator"

      @config.each do |sprite|
        next if sprite[:files].empty?
        system(bin,
               '--name', sprite[:name],
               '--scss-out', sprite[:scss_out],
               '--image-out', sprite[:img_out],
               *sprite[:files])
      end
    end

    private

    def parse_config(raw_config)
      raw_config.map do |key, config|
        files = config['source']
          .map { |glob_path| Dir.glob("app/assets/#{glob_path}") }
          .flatten
          .map { |p| File.absolute_path(p) }

        { name: config['name'] || key,
          scss_out: File.absolute_path("app/assets/#{config['scss_out']}"),
          img_out: File.absolute_path("app/assets/#{config['img_out']}"),
          files: files }
      end
    end
  end
end
