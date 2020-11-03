require 'spec_helper'
require 'support/editor'
require 'support/playground_actions'

RSpec.feature "Editor assistance for common code modifications", type: :feature, js: true do
  include PlaygroundActions

  before { visit '/' }

  scenario "building code without a main method offers adding one" do
    editor.set <<~EOF
      fn example() {}
    EOF
    click_on("Build")

    within('.output-warning') do
      click_on("add a main function")
    end

    within('.editor') do
      expect(editor).to have_line 'println!("Hello, world!")'
    end
  end

  scenario "using an unstable feature offers adding the feature flag" do
    in_channel_menu { click_on("Nightly") }
    editor.set <<~EOF
      fn foo<const T: usize>() {}
    EOF
    click_on("Build")

    within('.output-stderr') do
      click_on("add `#![feature(min_const_generics)]`")
    end

    within('.editor') do
      expect(editor).to have_line '#![feature(min_const_generics)]'
    end
  end

  scenario "using a type that hasn't been imported offers importing it" do
    editor.set <<~EOF
      fn example(_: HashMap) {}
    EOF
    click_on("Build")

    within('.output-stderr') do
      click_on("use std::collections::HashMap;")
    end

    within('.editor') do
      expect(editor).to have_line 'use std::collections::HashMap;'
    end
  end

  scenario "triggering a panic offers enabling backtraces" do
    editor.set <<~EOF
      fn main() {
          panic!("Oops");
      }
    EOF
    click_on("Run")

    within('.output-stderr') do
      click_on("run with `RUST_BACKTRACE=1` environment variable to display a backtrace")
    end

    within('.output-stderr') do
      expect(page).to have_content("stack backtrace:")
    end
  end

  private

  def editor
    Editor.new(page)
  end
end