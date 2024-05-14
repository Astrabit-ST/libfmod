# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("libfmod") do |ext|
  ext.lib_dir = "lib/libfmod"
end

task default: :compile
