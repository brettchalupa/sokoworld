#!/usr/bin/env ruby

# Script to deploy to itch

DRY_RUN = ARGV.include?("--dry-run")

def butler_push(build)
  butler_cmd = "butler push export/sokoworld-#{build}.zip brettchalupa/sokoworld:#{build}"
  if DRY_RUN
    puts "[DRY RUN] pushing to itch: #{butler_cmd}"
  else
    system(butler_cmd)
  end
end

puts `usagi export`
butler_push("linux")
butler_push("macos")
butler_push("windows")
butler_push("web")
