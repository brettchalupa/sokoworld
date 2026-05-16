#!/usr/bin/env ruby

# Script to deploy to itch

DRY_RUN = ARGV.include?("--dry-run")

def read_version
  src = File.read(File.expand_path("consts.lua", __dir__))
  m = src.match(/consts\.VERSION\s*=\s*"([^"]+)"/)
  raise "couldn't find consts.VERSION in consts.lua" unless m
  m[1]
end

VERSION = read_version

def butler_push(build)
  butler_cmd = "butler push --userversion #{VERSION} export/sokoworld-#{build}.zip brettchalupa/sokoworld:#{build}"
  if DRY_RUN
    puts "[DRY RUN] pushing to itch: #{butler_cmd}"
  else
    system(butler_cmd)
  end
end

puts "version: #{VERSION}"
puts `usagi export`
butler_push("linux")
butler_push("macos")
butler_push("windows")
butler_push("web")
