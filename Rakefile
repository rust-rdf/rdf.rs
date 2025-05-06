namespace :version do
  desc "Bump the version number"
  task :bump do
    old_version = File.read('VERSION').strip
    new_version = old_version.gsub(/\.\d+$/, &:succ)
    warn `git grep -l #{old_version} | xargs sd -F #{old_version} #{new_version}`.chomp
  end
end
