# TODO: http://stackoverflow.com/questions/7504744/how-do-i-hook-into-rails-after-files-are-reloaded-on-each-request-in-developme
# Mit to_prepare rusty sprite aufrufen und dann mit @import die dateien von /tmp/rustysprites importieren
module RustySprite
  class Engine < ::Rails::Engine
  end
end
