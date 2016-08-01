require 'sprockets'
require 'tilt'

require 'rusty_sprite/version'
require 'rusty_sprite/sprockets/transformer'

require 'rusty_sprite/engine' if defined?(::Rails::Engine)

module RustySprite
end
class SpriteDings < Tilt::Template
  def prepare
  end

  def evaluate context, _
    "$fsize: 26px; body.svenhuhu { font-size: $fsize; };"
  end
end


# ::Sprockets.register_mime_type 'text/css', extensions: ['.css.sprite']
::Sprockets.register_engine '.sprite', SpriteDings
