require 'spec_helper'

describe RustySprite do
  it 'has a version number' do
    expect(RustySprite::VERSION).not_to be nil
  end

  it 'does something useful' do
    require 'pry'
    binding.pry
    expect(false).to eq(true)
  end
end
