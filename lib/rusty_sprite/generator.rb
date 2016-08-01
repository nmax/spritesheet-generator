module RustySprite
  class Generator
    def self.call(name, files, scss_out, img_out)
      bin = "#{File.dirname(__FILE__)}/../rust/target/release/sprite-generator"
      absolute_files_paths =
        files.map { |path| File.absolute_path(path) }.join(' ')

      system(bin,
             '--name', name,
             '--scss-out', scss_out,
             '--image-out', img_out,
             absolute_files_paths)
    end
  end
end
