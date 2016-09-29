require 'spec_helper'
require 'yaml'

describe RustySprite::Generator do
  it 'parses a yaml config' do
    settings = <<~YAML
      foobar:
        scss_out: "sass_out.scss"
        img_out: "img_out.png"
        source: ["foo/bar/baz.png", "fizz/buzz/{3,5}.png"]
    YAML

    instance = described_class.new(YAML.load(settings))
    subject = instance.instance_variable_get('@config').first
    expect(subject[:files]).to eq []
    expect(subject[:name]).to eq 'foobar'
    ['img_out.png', `whoami`].each do |s|
      expect(subject[:img_out]).to include s.strip
    end

    ['sass_out.scss', `whoami`].each do |s|
      expect(subject[:scss_out]).to include s.strip
    end
    require 'pry'
    binding.pry
  end
end
