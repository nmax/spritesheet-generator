# encoding: utf-8
require 'yaml'

module RustySprite
  class Generator
    def initialize(config_path)
      img_root = 'app/assets/images'
      style_root = 'app/assets/stylesheets'

      @sprites = YAML.load_file(config_path).map do |key, config|
        files = config['source'].reduce([]) do |all_files, glob_path|
          all_files + Dir.glob("#{img_root}/#{glob_path}")
        end.map { |p| File.absolute_path(p) }

        { name: config['name'] || key,
          scss_out: "#{style_root}/#{config['scss_out']}",
          img_out: "#{img_root}/#{config['img_out']}",
          scss_img_url: config['img_out'],
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
               '--scss-img-url', sprite[:scss_img_url],
               '--image-out', sprite[:img_out],
               *sprite[:files])
      end
    end
  end
end
