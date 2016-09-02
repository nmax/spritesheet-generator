# encoding: utf-8
module RustySprite
  class Generator
    # TODO: Das könnte auch ein ffi Aufruf werden.
    def self.call(name, files, scss_out, img_out)
      bin =
        "#{File.dirname(__FILE__)}/../../rust/target/release/sprite-generator"

      system(bin,
             '--name', name,
             '--scss-out', File.absolute_path(scss_out),
             '--image-out', File.absolute_path(img_out),
             files.map { |path| File.absolute_path(path) }.join(' '))
    end
  end
end
