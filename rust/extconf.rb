if !system('cargo --version') || !system('rustc --version')
  raise 'You have to install Rust with Cargo.'
end

