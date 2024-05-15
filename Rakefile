# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("libfmod_ext") do |ext|
  ext.lib_dir = "lib/libfmod"
end


desc "Compile the extension with debug symbols"
task "compile:debug" do
  ENV["RB_SYS_CARGO_PROFILE"] = "dev"
  Rake::Task["compile"].invoke
end

task default: :compile
